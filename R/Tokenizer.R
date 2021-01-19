#' @export
tokenizer <- R6::R6Class(
  "tokenizer",
  public = list(
    ptr = NULL,
    initialize = function(model) {
      self$ptr <- .Call(wrap__RTokenizer__from_model, model$ptr, PACKAGE = "helloextendr")
    },
    train = function(files) {
      .Call(wrap__RTokenizer__train, self$ptr, files, PACKAGE = "helloextendr")
      invisible(self)
    },
    encode = function(sequence, add_special_tokens = FALSE) {
      .Call(wrap__RTokenizer__encode, self$ptr, sequence, add_special_tokens, PACKAGE = "helloextendr")
    }
  )
)