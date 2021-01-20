#' @export
tokenizer <- R6::R6Class(
  "tokenizer",
  public = list(
    ptr = NULL,
    initialize = function(model) {
      self$ptr <- .Call(wrap__RTokenizer__from_model, model$ptr, PACKAGE = "hftokenizers")
    },
    train = function(files) {
      .Call(wrap__RTokenizer__train, self$ptr, files, PACKAGE = "hftokenizers")
      invisible(self)
    },
    encode = function(sequence, add_special_tokens = FALSE) {
      .Call(wrap__RTokenizer__encode, self$ptr, sequence, add_special_tokens, PACKAGE = "hftokenizers")
    },
    # TODO is this the correct default?
    get_vocab = function(with_added_tokens = FALSE) {
      .Call(wrap__RTokenizer__get_vocab, self$ptr, with_added_tokens, PACKAGE = "hftokenizers")
    }
  )
)