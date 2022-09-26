# implfuture

My personal website, hosted at [implfuture.dev](https://implfuture.dev).

# Building / Running

```bash
# local development
ibazel run //server --//:show_drafts

# build and deploy container image
bazel run -c opt //server:push-amd64
```

In order to push to ECR, make sure `~/.docker/config.json` contains:

```json
{
  "credsStore": "ecr-login"
}
```
