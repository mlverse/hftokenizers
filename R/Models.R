#' @export
models_bpe <- R6::R6Class(
  "model_bpe",
  public = list(
    ptr = NULL,
    initialize = function(vocab = NULL, merges = NULL, dropout = NA, 
                          unk_token = NA) {
      self$ptr <- .Call(wrap__RModelsBpe__new, vocab, merges, dropout, unk_token, PACKAGE = "helloextendr")
    }  
  )
)


