RTokenizerBuilder <- R6::R6Class(
  classname = "RTokenizerBuilder",
  public = list(
    p = NULL,
    initialize = function() {
      self$p <- .Call(wrap__RTokenizerBuilder__new)
    }
  )
)