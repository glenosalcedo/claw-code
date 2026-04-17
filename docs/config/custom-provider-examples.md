# Custom Provider Examples

This document provides configuration examples for connecting claw-code to third-party AI providers via the `providers` feature in `.claw.json`. Use these examples when you need to route requests to models not served by OpenAI's default endpoint, or when you prefer a specific provider's pricing, latency, or model catalog. Ensure the required environment variable is exported in your shell before running `claw` — the tool reads it at runtime and will fail with an authentication error if the variable is missing or empty.

## OpenRouter

OpenRouter aggregates dozens of model providers behind a single OpenAI-compatible API. Visit the [keys page](https://openrouter.ai/settings/keys) to generate an API key.

```env
export OPENROUTER_API_KEY="sk-or-..."
```

```json
{
  "openrouter": {
    "base_url": "https://openrouter.ai/api/v1",
    "api_key_env": "OPENROUTER_API_KEY",
    "models": [
      "anthropic/claude-sonnet-4.6",
      "google/gemini-2.5-pro",
      "meta-llama/llama-3.3-70b-instruct"
    ]
  }
}
```

```bash
claw --model openrouter/anthropic/claude-sonnet-4.6 "Explain quantum entanglement in one sentence."
```

## Ollama

Ollama runs locally on your machine and exposes an OpenAI-compatible endpoint. Download from [ollama.ai](https://ollama.ai) and start the server with `ollama serve`. The API key variable must be set to any non-empty value; Ollama ignores it but claw-code requires the environment variable to exist.

```env
export OLLAMA_API_KEY="any-value"
```

```json
{
  "ollama": {
    "base_url": "http://localhost:11434/v1",
    "api_key_env": "OLLAMA_API_KEY",
    "models": [
      "llama3.3:70b",
      "qwen2.5-coder:32b",
      "deepseek-r1:70b"
    ]
  }
}
```

```bash
claw --model ollama/llama3.3:70b "Summarize this file: README.md"
```

## LM Studio

LM Studio provides a local server that loads GGUF models and exposes an OpenAI-compatible API. Download from [lmstudio.ai](https://lmstudio.ai) and start the server from the UI (Server tab). The API key variable must be set to any non-empty value; the server ignores it but claw-code requires the environment variable to exist.

```env
export LMSTUDIO_API_KEY="any-value"
```

```json
{
  "lmstudio": {
    "base_url": "http://localhost:1234/v1",
    "api_key_env": "LMSTUDIO_API_KEY",
    "models": [
      "lmstudio-community/meta-llama-3.1-8b-instruct-gguf",
      "lmstudio-community/qwen2.5-coder-14b-instruct-gguf"
    ]
  }
}
```

```bash
claw --model lmstudio/lmstudio-community/meta-llama-3.1-8b-instruct-gguf "What does this regex match: ^\\d{3}-\\d{4}$?"
```

## Groq

Groq offers low-latency inference for open-source models. Obtain an API key from the [console](https://console.groq.com/keys).

```env
export GROQ_API_KEY="gsk_..."
```

```json
{
  "groq": {
    "base_url": "https://api.groq.com/openai/v1",
    "api_key_env": "GROQ_API_KEY",
    "models": [
      "llama-3.3-70b-versatile",
      "mixtral-8x7b-32768",
      "deepseek-r1-distill-llama-70b"
    ]
  }
}
```

```bash
claw --model groq/llama-3.3-70b-versatile "Give me a one-word greeting."
```

## Together.ai

Together.ai hosts fine-tuned and open-source models on fast infrastructure. Get an API key from [together.ai](https://together.ai).

```env
export TOGETHER_API_KEY="..."
```

```json
{
  "together": {
    "base_url": "https://api.together.xyz/v1",
    "api_key_env": "TOGETHER_API_KEY",
    "models": [
      "meta-llama/Llama-3.3-70B-Instruct-Turbo",
      "Qwen/Qwen2.5-Coder-32B-Instruct",
      "deepseek-ai/DeepSeek-V3"
    ]
  }
}
```

```bash
claw --model together/meta-llama/Llama-3.3-70B-Instruct-Turbo "Translate 'hello' to French."
```

## Fireworks AI

Fireworks AI provides high-throughput inference for open-source models. Generate a key at [fireworks.ai](https://fireworks.ai).

```env
export FIREWORKS_API_KEY="..."
```

```json
{
  "fireworks": {
    "base_url": "https://api.fireworks.ai/inference/v1",
    "api_key_env": "FIREWORKS_API_KEY",
    "models": [
      "accounts/fireworks/models/llama-v3p3-70b-instruct",
      "accounts/fireworks/models/qwen2p5-coder-32b-instruct"
    ]
  }
}
```

```bash
claw --model fireworks/accounts/fireworks/models/llama-v3p3-70b-instruct "List three benefits of unit testing."
```

## DeepInfra

DeepInfra offers serverless inference for popular open-source models. Obtain a key at [deepinfra.com](https://deepinfra.com).

```env
export DEEPINFRA_API_KEY="..."
```

```json
{
  "deepinfra": {
    "base_url": "https://api.deepinfra.com/v1/openai",
    "api_key_env": "DEEPINFRA_API_KEY",
    "models": [
      "meta-llama/Meta-Llama-3.1-70B-Instruct",
      "Qwen/Qwen2.5-Coder-32B-Instruct"
    ]
  }
}
```

```bash
claw --model deepinfra/meta-llama/Meta-Llama-3.1-70B-Instruct "What is the capital of Japan?"
```

## Anyscale

Anyscale provides managed endpoints for open-source models. Get an API key from [anyscale.com](https://anyscale.com).

```env
export ANYSCALE_API_KEY="..."
```

```json
{
  "anyscale": {
    "base_url": "https://api.endpoints.anyscale.com/v1",
    "api_key_env": "ANYSCALE_API_KEY",
    "models": [
      "meta-llama/Meta-Llama-3.1-70B-Instruct"
    ]
  }
}
```

```bash
claw --model anyscale/meta-llama/Meta-Llama-3.1-70B-Instruct "Explain what a goroutine is in Go."
```

## vLLM

vLLM is a high-performance local inference server. Start it with `vllm serve <model> --api-key <your-key>` (see the [vLLM documentation](https://docs.vllm.ai)). The API key variable must match the value you pass to vLLM; claw-code sends this key in the Authorization header.

```env
export VLLM_API_KEY="your-vllm-key"
```

```json
{
  "vllm": {
    "base_url": "http://localhost:8000/v1",
    "api_key_env": "VLLM_API_KEY",
    "models": [
      "meta-llama/Llama-3.3-70B-Instruct"
    ]
  }
}
```

```bash
claw --model vllm/meta-llama/Llama-3.3-70B-Instruct "Describe the purpose of the .gitignore file."
```

## Combining providers

The following JSON block demonstrates a complete `.claw.json` providers section that includes all nine providers. The map key (e.g., `openrouter`, `ollama`) becomes the prefix used in `--model` arguments. When you run `claw --model`, TAB completion will show models from all configured providers, making it easy to switch between providers without editing the config file.

```json
{
  "providers": {
    "openrouter": {
      "base_url": "https://openrouter.ai/api/v1",
      "api_key_env": "OPENROUTER_API_KEY",
      "models": [
        "anthropic/claude-sonnet-4.6",
        "google/gemini-2.5-pro",
        "meta-llama/llama-3.3-70b-instruct"
      ]
    },
    "ollama": {
      "base_url": "http://localhost:11434/v1",
      "api_key_env": "OLLAMA_API_KEY",
      "models": [
        "llama3.3:70b",
        "qwen2.5-coder:32b",
        "deepseek-r1:70b"
      ]
    },
    "lmstudio": {
      "base_url": "http://localhost:1234/v1",
      "api_key_env": "LMSTUDIO_API_KEY",
      "models": [
        "lmstudio-community/meta-llama-3.1-8b-instruct-gguf",
        "lmstudio-community/qwen2.5-coder-14b-instruct-gguf"
      ]
    },
    "groq": {
      "base_url": "https://api.groq.com/openai/v1",
      "api_key_env": "GROQ_API_KEY",
      "models": [
        "llama-3.3-70b-versatile",
        "mixtral-8x7b-32768",
        "deepseek-r1-distill-llama-70b"
      ]
    },
    "together": {
      "base_url": "https://api.together.xyz/v1",
      "api_key_env": "TOGETHER_API_KEY",
      "models": [
        "meta-llama/Llama-3.3-70B-Instruct-Turbo",
        "Qwen/Qwen2.5-Coder-32B-Instruct",
        "deepseek-ai/DeepSeek-V3"
      ]
    },
    "fireworks": {
      "base_url": "https://api.fireworks.ai/inference/v1",
      "api_key_env": "FIREWORKS_API_KEY",
      "models": [
        "accounts/fireworks/models/llama-v3p3-70b-instruct",
        "accounts/fireworks/models/qwen2p5-coder-32b-instruct"
      ]
    },
    "deepinfra": {
      "base_url": "https://api.deepinfra.com/v1/openai",
      "api_key_env": "DEEPINFRA_API_KEY",
      "models": [
        "meta-llama/Meta-Llama-3.1-70B-Instruct",
        "Qwen/Qwen2.5-Coder-32B-Instruct"
      ]
    },
    "anyscale": {
      "base_url": "https://api.endpoints.anyscale.com/v1",
      "api_key_env": "ANYSCALE_API_KEY",
      "models": [
        "meta-llama/Meta-Llama-3.1-70B-Instruct"
      ]
    },
    "vllm": {
      "base_url": "http://localhost:8000/v1",
      "api_key_env": "VLLM_API_KEY",
      "models": [
        "meta-llama/Llama-3.3-70B-Instruct"
      ]
    }
  }
}
```
