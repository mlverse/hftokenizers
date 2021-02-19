test_that("trainers_bpe", {
  
  trainer <- trainers_bpe$new()
  expect_s3_class(trainer, c("trainers_bpe", "Trainer"))
  
})
