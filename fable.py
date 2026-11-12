import os
import sys
import boto3

# Kimlik bilgisi Codespaces Secret'lardan gelir (koda yazılmaz):
# AWS_ACCESS_KEY_ID ve AWS_SECRET_ACCESS_KEY
if not (os.environ.get("AWS_ACCESS_KEY_ID") and os.environ.get("AWS_SECRET_ACCESS_KEY")):
    sys.exit("HATA: AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY secret'ları bulunamadı.")

client = boto3.client("bedrock-runtime", region_name="us-east-1")

soru = " ".join(sys.argv[1:]) or "Merhaba! Kendini bir cümleyle tanıt."

yanit = client.converse(
    modelId="global.anthropic.claude-fable-5-2",
    messages=[{"role": "user", "content": [{"text": soru}]}],
    inferenceConfig={"maxTokens": 1000},
)

print(yanit["output"]["message"]["content"][0]["text"])
