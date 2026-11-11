import os
import sys
import boto3

# API anahtarı koda yazılmaz; Codespaces Secret'tan gelir.
if not os.environ.get("AWS_BEARER_TOKEN_BEDROCK"):
    sys.exit("HATA: AWS_BEARER_TOKEN_BEDROCK secret'ı bulunamadı.")

client = boto3.client("bedrock-runtime", region_name="us-east-1")

soru = " ".join(sys.argv[1:]) or "Merhaba! Kendini bir cümleyle tanıt."

yanit = client.converse(
    modelId="global.anthropic.claude-fable-5-2",
    messages=[{"role": "user", "content": [{"text": soru}]}],
    inferenceConfig={"maxTokens": 1000},
)

print(yanit["output"]["message"]["content"][0]["text"])
