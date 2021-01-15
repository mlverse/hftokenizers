RModelWrapper <- R6::R6Class(
  "RModelWrapper",
  public = list(
    p = NULL,
    initialize = function() {
      self$p <- .Call(wrap__RModelWrapper__new)
    }  
  )
)