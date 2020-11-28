#lang info

(define collection "egg-herbie-redirect")
(define version "1.4")

(define pkg-desc "Racket bindings for simplifying math expressions using egg")

(define deps
  '(("egg-herbie-windows" #:platform "win32\\x86_64" #:version "1.4")
    ("egg-herbie-osx" #:platform "x86_64-macosx" #:version "1.4")
    ("egg-herbie-linux" #:platform "x86_64-linux" #:version "1.4")))

(define pkg-authors
  `("Oliver Flatt"))
