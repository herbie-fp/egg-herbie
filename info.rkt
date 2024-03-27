#lang info

(define collection "egg-herbie-redirect")
(define version "2.0")

(define pkg-desc "Racket bindings for simplifying math expressions using egg")

(define deps
  '(("egg-herbie-osx" #:platform "x86_64-macosx" #:version "2.0")
    ("egg-herbie-windows" #:platform "win32\\x86_64" #:version "2.0")
    ("egg-herbie-linux" #:platform "x86_64-linux" #:version "2.0")
    ("egg-herbie-linux" #:platform "x86_64-linux-natipkg" #:version "2.0")))   ; Dockerfile

(define pkg-authors
  `("Oliver Flatt"
    "Brett Saiki"))
