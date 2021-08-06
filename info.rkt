#lang info

(define collection "egg-herbie-redirect")
(define version "1.5")

(define pkg-desc "Racket bindings for simplifying math expressions using egg")

; Herbie Dockerfile uses 'x86_64-linux-natipkg'
(define deps
  '(("egg-herbie-windows" #:platform #rx"win32\\x86_64*" #:version "1.5")
    ("egg-herbie-osx" #:platform #rx"x86_64-macosx*" #:version "1.5")
    ("egg-herbie-linux" #:platform #rx"x86_64-linux*" #:version "1.5")))

(define pkg-authors
  `("Oliver Flatt"))
