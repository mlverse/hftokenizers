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
    encode = function(sequence, pair = NULL, is_pretokenized = FALSE, add_special_tokens = TRUE) {
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
    },
    encode_batch = function(inputs, is_pre_tokenized = FALSE, add_special_tokens = TRUE) {
      if (!is.list(inputs))
        inputs <- list(inputs)
      lapply(
        self$obj$encode_batch(inputs, add_special_tokens),
        encoding$new
      )
    },
    #' @description 
    #' Enable the padding
    #' @param direction (str, optional, defaults to right) – The direction in which
    #'   to pad. Can be either right or left
    #' @param pad_to_multiple_of (int, optional) – If specified, the padding 
    #'   length should always snap to the next multiple of the given value. 
    #'   For example if we were going to pad with a length of 250 but 
    #'   `pad_to_multiple_of=8` then we will pad to 256.
    #' @param pad_id (int, defaults to 0) – The id to be used when padding
    #' @param pad_type_id (int, defaults to 0) – The type id to be used when padding
    #' @param pad_token (str, defaults to `[PAD]`) – The pad token to be used when padding
    #' @param length (int, optional) – If specified, the length at which to pad. 
    #'   If not specified we pad using the size of the longest sequence in a batch.
    #'
    enable_padding = function(direction='right', pad_id=0, pad_type_id=0, 
                              pad_token='[PAD]', length=NULL, 
                              pad_to_multiple_of=NULL) {
      self$obj$enable_padding(direction, pad_id, pad_type_id, pad_token, length,
                              pad_to_multiple_of)
      invisible(self)
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