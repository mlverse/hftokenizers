test_that("Can create a tokenizer and train", {
  
  expect_error(tok <- tokenizer$new(models_bpe$new()), regex = NA)
  expect_error(tok$train("assets/small.txt"), regex = NA)
  expect_error(o <- tok$encode("hello world"), regex = NA)
  expect_equal(class(o), "integer")
  
})
