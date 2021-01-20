#' @export
models_bpe <- R6::R6Class(
  "model_bpe",
  public = list(
    ptr = NULL,
    initialize = function(vocab = NULL, merges = NULL, cache_capacity=NA, 
                          dropout=NA, unk_token=NA, continuing_subword_prefix=NA, 
                          end_of_word_suffix=NA, fuse_unk=NA) {
      self$ptr <- .Call(wrap__RModelsBpe__new, vocab, merges, cache_capacity, 
                        dropout, unk_token, continuing_subword_prefix, 
                        end_of_word_suffix, fuse_unk, 
                        PACKAGE = "hftokenizers")
    }  
  )
)


