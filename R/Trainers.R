trainer <- R6::R6Class(
  "Trainer",
  public = list(
    obj = NULL,
    initialize = function(obj) {
      self$obj = obj
    }
  )
)


#' Byte pair encoding trainer class
#'
trainers_bpe <- R6::R6Class(
  "trainers_bpe",
  inherit = trainer,
  public = list(
    #' @description  
    #' Trainer capable of training a BPE model
    #' 
    #' @param vocab_size (int, optional) – The size of the final vocabulary, including all tokens and alphabet.
    #' @param min_frequency (int, optional) – The minimum frequency a pair should have in order to be merged.
    #' @param show_progress (bool, optional) – Whether to show progress bars while training.
    #' @param special_tokens (List[Union[str, AddedToken]], optional) – A list of special tokens the model should know of.
    #' @param limit_alphabet (int, optional) – The maximum different characters to keep in the alphabet.
    #' @param initial_alphabet (List[str], optional) – A list of characters to include in the initial alphabet, even if not seen in the training dataset. If the strings contain more than one character, only the first one is kept.
    #' @param continuing_subword_prefix (str, optional) – A prefix to be used for every subword that is not a beginning-of-word.
    #' @param end_of_word_suffix (str, optional) – A suffix to be used for every subword that is a end-of-word.
    #' 
    #' @export
    initialize = function(
      vocab_size=NULL, 
      min_frequency=NULL, 
      show_progress=NULL, 
      special_tokens=NULL, 
      limit_alphabet=NULL, 
      initial_alphabet=NULL, 
      continuing_subword_prefix=NULL, 
      end_of_word_suffix=NULL
    ) {
      self$obj <- RBpeTrainer$new(
        vocab_size,
        min_frequency,
        show_progress,
        special_tokens,
        limit_alphabet,
        initial_alphabet,
        continuing_subword_prefix,
        end_of_word_suffix
      )
    }
  )
)
