import boto3

REGION = "us-east-1"
bedrock = boto3.client("bedrock", region_name=REGION)
runtime = boto3.client("bedrock-runtime", region_name=REGION)

# 1) Hesabin bolgesinde gorunen tum Anthropic profilleri
print("=== Gorunen Anthropic profilleri ===")
gorunen = []
token = None
while True:
    kw = {"maxResults": 100}
    if token:
        kw["nextToken"] = token
    r = bedrock.list_inference_profiles(**kw)
    for p in r["inferenceProfileSummaries"]:
        if "anthropic" in p["inferenceProfileId"]:
            gorunen.append(p["inferenceProfileId"])
    token = r.get("nextToken")
    if not token:
        break
for g in sorted(gorunen):
    print(" ", g)

# 2) En ustten asagiya adaylari gercekten cagirip dene (cok kucuk istek)
adaylar = [
    "global.anthropic.claude-fable-5-2",
    "global.anthropic.claude-fable-5-1",
    "global.anthropic.claude-opus-5-5",
    "global.anthropic.claude-opus-5",
    "global.anthropic.claude-sonnet-5-5",
    "global.anthropic.claude-opus-4-8",
    "global.anthropic.claude-opus-4-6",
    "global.anthropic.claude-sonnet-4-6",
]
print("\n=== Deneme (ustten asagi) ===")
calisan = []
for m in adaylar:
    try:
        runtime.converse(
            modelId=m,
            messages=[{"role": "user", "content": [{"text": "1+1?"}]}],
            inferenceConfig={"maxTokens": 5},
        )
        print("CALISIYOR :", m)
        calisan.append(m)
    except Exception as e:
        print("calismiyor:", m, "->", str(e)[:90])

print("\nKullanabilecegin en ust model:", calisan[0] if calisan else "hicbiri calismadi")
