#' The tokenizer class
#' 
tokenizer <- R6::R6Class(
  "tokenizer",
  public = list(
    obj = NULL,
    initialize = function(model) {
      self$obj <- RTokenizer$from_model(model$ptr)
    },
    train = function(files) {
      self$obj$train(files)
      invisible(self)
    },
    encode = function(sequence, pair = NULL, is_pretokenized = FALSE, add_special_tokens = FALSE) {
      self$obj$encode(sequence, pair, is_pretokenized, add_special_tokens)
    },
    decode = function(ids, skip_special_tokens=TRUE) {
      self$obj$decode(ids, skip_special_tokens)
    },
    # TODO is this the correct default?
    get_vocab = function(with_added_tokens = FALSE) {
      self$obj$get_vocab(with_added_tokens)
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
      if (missing(x)) {
        self$obj$get_pre_tokenizer()
      } else {
        self$obj$set_pre_tokenizer(pre_tokenizer)
      }
    }
  )
)