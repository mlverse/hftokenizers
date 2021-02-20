encoding <- R6::R6Class(
  "encoding",
  public = list(
    obj = NULL,
    initialize = function(obj) {
      self$obj <- obj
    }
  ),
  active = list(
    tokens = function(tokens) {
      if (missing(tokens))
        self$obj$get_tokens()
      else
        stop("Can't set the tokens attribute.")
    },
    ids = function(ids) {
      if (missing(ids))
        self$obj$get_ids()
      else
        stop("Can't set the ids attribute.")
    },
    offsets = function(offsets) {
      if (missing(offsets))
        self$obj$get_offsets()
      else
        stop("Can't set the offsets attribute.")
    }
  )
)