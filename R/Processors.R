post_processors <- R6::R6Class(
  "post_processors",
  public = list(
    obj = NULL,
    initialize = function(obj) {
      self$obj <- obj
    },
    process = function(encoding, pair_encoding = NULL, add_special_tokens = TRUE) {
      encoding$new(
        self$obj$process(
          encoding$obj, 
          pair_encoding$obj, 
          add_special_tokens
        )  
      )
    }
  )
)


#' Template Processing
#' 
#' Provides a way to specify templates in order to add the special tokens to each 
#' input sequence as relevant.
#'
#' @export
post_processors_template_processing <- R6::R6Class(
  "processors_template_processing",
  inherit = post_processors,
  public = list(
    #' @param single (Template) – The template used for single sequences
    #' @param pair (Template) – The template used when both sequences are specified
    #' @param special_tokens (Tokens) – The list of special tokens used in each sequences
    initialize = function(single = NULL, pair = NULL, special_tokens = NULL) {
      super$initialize(
        RTemplateProcessing$new(
          single = single, 
          pair = pair, 
          special_tokens = special_tokens
        )
      )
    }
  )
)