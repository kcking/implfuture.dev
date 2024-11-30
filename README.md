# implfuture

My personal website, hosted at [implfuture.dev](https://implfuture.dev).

# Building / Running

```bash
# local development
ibazel run //server --//:show_drafts

# build and deploy container image
bazel run -c opt //server:push-amd64 --stamp
```

In order to push to ECR, make sure `~/.docker/config.json` contains:

```json
{
  "credsStore": "ecr-login"
}
```

To authenticate with podman, use
```bash
aws ecr get-login-password --region us-west-1 | podman login --username AWS --password-stdin 689191389309.dkr.ecr.us-west-1.amazonaws.com
```
with config
```json
{
	"auths": {},
	"credsStore": "ecr-login",
	"currentContext": "desktop-linux"
}
```
