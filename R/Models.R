model <- R6::R6Class(
  "Model",
  public = list(
    ptr = NULL,
    initialize = function(ptr) {
      self$ptr = ptr
    },
    save = function(folder, prefix = NA) {
      .Call(wrap__RModel__save, self$ptr, folder, prefix)
    }
  )
)

#' @export
models_bpe <- R6::R6Class(
  "model_bpe",
  inherit = model,
  public = list(
    ptr = NULL,
    initialize = function(vocab = NULL, merges = NULL, cache_capacity=NA, 
                          dropout=NA, unk_token=NA, continuing_subword_prefix=NA, 
                          end_of_word_suffix=NA, fuse_unk=NA, ...) {
      if (!is.null(list(...)$ptr)) {
        ptr <- list(...)$ptr
      } else {
        ptr <- .Call(wrap__RModelsBpe__new, vocab, merges, cache_capacity, 
                     dropout, unk_token, continuing_subword_prefix, 
                     end_of_word_suffix, fuse_unk, 
                     PACKAGE = "hftokenizers")  
      }
      
      super$initialize(ptr)
    }
  )
)

models_bpe$read_file <- function(vocab, merges) {
  .Call(wrap__RModelsBpe__read_file, vocab, merges)
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
