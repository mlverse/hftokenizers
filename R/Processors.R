#' Template Processing
#' 
#' Provides a way to specify templates in order to add the special tokens to each 
#' input sequence as relevant.
#'
#' @export
post_processors_template_processing <- R6::R6Class(
  "processors_template_processing",
  public = list(
    obj = NULL,
    #' @param single (Template) – The template used for single sequences
    #' @param pair (Template) – The template used when both sequences are specified
    #' @param special_tokens (Tokens) – The list of special tokens used in each sequences
    initialize = function(single = NULL, pair = NULL, special_tokens = NULL) {
      self$obj <- RTemplateProcessing$new(single = single, pair = pair, special_tokens = special_tokens)
    }
  )
)