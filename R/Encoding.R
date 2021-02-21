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
    type_ids = function(type_ids) {
      if (missing(type_ids))
        self$obj$get_type_ids()
      else
        stop("Can't set the type_ids attribute.")
    },
    offsets = function(offsets) {
      if (missing(offsets))
        self$obj$get_offsets()
      else
        stop("Can't set the offsets attribute.")
    },
    attention_mask = function(attention_mask) {
      if (missing(attention_mask))
        self$obj$get_attention_mask()
      else
        stop("Can't set the attention_mask attribute.")
    }
  )
)