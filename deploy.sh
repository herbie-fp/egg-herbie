#!/bin/sh

setup_git() {
    git config --global user.email "travis@travis-ci.org"
    git config --global user.name "Travis CI"
}

commit_website_files() {
    git checkout -b egg-herbie-deploy-$TRAVIS_OS_NAME
    git add -u
    git add target/release/* -f
    git add .travis.yml
    git commit --message "Travis build: $TRAVIS_BUILD_NUMBER"
}

upload_files() {
    git remote add origin-pages https://${GITHUB_TOKEN}@github.com/oflatt/egg-herbie > /dev/null 2>&1
    git fetch origin-pages egg-herbie-deploy-$TRAVIS_OS_NAME
    git merge origin-pages/egg-herbie-deploy-$TRAVIS_OS_NAME -s ours
    git commit
    git push --set-upstream origin-pages egg-herbie-deploy-$TRAVIS_OS_NAME
}

setup_git
commit_website_files
upload_files
