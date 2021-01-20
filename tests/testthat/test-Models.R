test_that("bpe", {
  
  model <- models_bpe$new()
  
  tmp <- tempfile()
  dir.create(tmp)
  
  files <- model$save(tmp, "hi")  
  
  expect_equal(
    sort(files),
    sort(list.files(tmp, full.names = TRUE))
  )
  
})
