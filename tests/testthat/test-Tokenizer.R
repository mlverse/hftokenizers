test_that("Can create a tokenizer and train", {
  
  expect_error(tok <- tokenizer$new(models_bpe$new()), regex = NA)
  expect_error(tok$train("assets/small.txt"), regex = NA)
  expect_error(o <- tok$encode("hello world"), regex = NA)
  expect_equal(class(o), "integer")
  expect_error(o <- tok$encode(c("hello", "world"), is_pre_tokenized = TRUE), regex = NA)
  expect_equal(class(o), "integer")
  
  expect_error(o <- tok$encode("hi"), regex = NA)
  expect_equal(tok$decode(o, FALSE), "hi")
  
  
  expect_error(tok$encode(c("hello", "world")))
  
  
  tokens <- tok$get_vocab()
  expect_named(tokens)
  expect_equal(class(tokens), "integer")

})
