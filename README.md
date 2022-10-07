# Intro

See https://club.black.co.at/log/posts/2022-09-26-unversal-pipeline-compiler/index.html for what this project is trying to do.

# Evaluating output options

## templating

### askama 

+ provides for a type-safe data -> template injection
- doesn't provide a built-in way for shared data
+ this can be easily worked around by using a `data: SharedData` member on each template struct
+ full control over output, including comments
+ independent of target file syntax
- requires in-depth understanding of the target file-format
- using special target format features like multi-line strings is awkward

## serialization

### serde(_yaml)

- doesn't allow inserting comments
- requires output format and target specific data structures
+ no additional coding on the template side required

# Other resources
* https://github.com/japaric/trust: Travis CI and AppVeyor template to test your Rust crate on 5 architectures and publish binary releases of it for Linux, macOS and Windows
* https://docs.gitlab.com/ee/ci/
* https://docs.github.com/en/actions/quickstart
* https://circleci.com/docs/configuration-reference
