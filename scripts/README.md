# AI Setup (Optional)

The project supports local AI tagging using Ollama.

AI is NOT required for development.

Default mode uses a mock service.

## Start AI service

`./scripts/dev-ai`

## Stop AI Service

`./scripts/dev-ai-stop`

## Verify

curl http://localhost:11434/api/tags

## Gotchas

If you aren't able to run those two scripts above run these first:

```
chmod +x scripts/dev-ai
chmod +x scripts/dev-ai-stop
```

## Environment

AI_PROVIDER=ollama
OLLAMA_BASE_URL=http://localhost:11434
OLLAMA_MODEL=llava