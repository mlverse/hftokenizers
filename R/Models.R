#' @export
models_bpe <- R6::R6Class(
  "model_bpe",
  public = list(
    ptr = NULL,
    initialize = function(vocab = NULL, merges = NULL, cache_capacity=NA, 
                          dropout=NA, unk_token=NA, continuing_subword_prefix=NA, 
                          end_of_word_suffix=NA, fuse_unk=NA, ...) {
      if (!is.null(list(...)$ptr)) {
        self$ptr <- list(...)$ptr
        return(invisible(self))
      }
      
      .Call(wrap__RModelsBpe__new, vocab, merges, cache_capacity, 
            dropout, unk_token, continuing_subword_prefix, 
            end_of_word_suffix, fuse_unk, 
            PACKAGE = "hftokenizers")
        
    }
  )
)

models_bpe$read_file <- function(vocab, merges) {
  .Call(wrap__RModelsBpe_read_file, vocab, merge)
}

models_bpe$from_file <- function(vocab, merges, cache_capacity=NA, 
                                 dropout=NA, unk_token=NA, continuing_subword_prefix=NA, 
                                 end_of_word_suffix=NA, fuse_unk=NA) {
  models_bpe$new(ptr = .Call(wrap__RModelsBpe__from_file, vocab, merges, cache_capacity, 
                             dropout, unk_token, continuing_subword_prefix, 
                             end_of_word_suffix, fuse_unk, 
                             PACKAGE = "hftokenizers")
  )
}
