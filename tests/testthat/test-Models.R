test_that("bpe", {
  
  model <- models_bpe$new()
  
  tmp <- tempfile()
  dir.create(tmp)
  
  files <- model$save(tmp, "hi")  
  
  expect_equal(
    sort(normalizePath(files)),
    sort(normalizePath(list.files(tmp, full.names = TRUE)))
  )
  
  # reload files
  expect_error(
    model <- models_bpe$from_file(files[1], files[2]),
    regexp = NA
  )
  
  # read file
  expect_error(
    obj <- models_bpe$read_file(files[1], files[2]),
    regexp = NA
  )
  
  
})
