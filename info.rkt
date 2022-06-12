#lang info

(define collection "egg-herbie-redirect")
(define version "1.6")

(define pkg-desc "Racket bindings for simplifying math expressions using egg")

(define deps
  '(("egg-herbie-osx" #:platform "x86_64-macosx" #:version "1.6")
    ("egg-herbie-windows" #:platform "win32\\x86_64" #:version "1.6")
    ("egg-herbie-linux" #:platform "x86_64-linux" #:version "1.6")
    ("egg-herbie-linux" #:platform "x86_64-linux-natipkg" #:version "1.6")))   ; Dockerfile

(define pkg-authors
  `("Oliver Flatt"
    "Brett Saiki"))
