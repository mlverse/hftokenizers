null_or_obj <- function(x) {
  if (is.null(x))
    x
  else
    x$obj
}

#' The tokenizer class
#' @export
tokenizer <- R6::R6Class(
  "tokenizer",
  public = list(
    obj = NULL,
    initialize = function(model) {
      if (inherits(model, "RTokenizer")) {
        self$obj <- model
        return()
      }
      self$obj <- RTokenizer$from_model(model$ptr)
    },
    train = function(files, trainer = NULL) {
      self$obj$train(files, null_or_obj(trainer))
      invisible(self)
    },
    encode = function(sequence, pair = NULL, is_pretokenized = FALSE, add_special_tokens = FALSE) {
      encoding$new(self$obj$encode(sequence, pair, is_pretokenized, add_special_tokens))
    },
    decode = function(ids, skip_special_tokens=TRUE) {
      self$obj$decode(ids, skip_special_tokens)
    },
    # TODO is this the correct default?
    get_vocab = function(with_added_tokens = FALSE) {
      self$obj$get_vocab(with_added_tokens)
    },
    save = function(path, pretty = FALSE) {
      self$obj$save(path.expand(path), pretty)
    },
    token_to_id = function(token) {
      self$obj$token_to_id(token)
    }
  ),
  active = list(
    
    #' @description 
    #' Get and set the pre_tokenizer
    #'
    #' @param pre_tokenizer if missing returns the pre tokenizer otherwise sets
    #'  it in the tokenizer.
    #'
    pre_tokenizer = function(pre_tokenizer) {
      if (missing(pre_tokenizer)) {
        self$obj$get_pre_tokenizer()
      } else {
        self$obj$set_pre_tokenizer(pre_tokenizer)
      }
    },
    
    post_processor = function(post_processor) {
      if (missing(post_processor)) {
        self$obj$get_post_processor()
      } else {
        self$obj$set_post_processor(post_processor$obj)
      }
    }
  )
)

tokenizer$from_file <- function(path) {
  tokenizer$new(RTokenizer$from_file(path.expand(path)))
}