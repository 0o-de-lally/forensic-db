#!/bin/bash

# Cloudflare R2 Credentials (Public Read-Only Mirror)
# Source: https://github.com/0LNetworkCommunity/libra-archive-mirrors
access_key="f60833651da1504ecdff70053e6a5120"
secret_key="8c1e4ed3724e6a5c2bc41059f0041900788ad9f406f39a3e30d83231288db717"
account_id="afcdb03dd0764818ac9aec7fe0c0b8b5"
bucket="libra-archives"
host="${account_id}.r2.cloudflarestorage.com"
endpoint="https://${host}"
region="auto"
service="s3"

# Date and Time
date=$(date -u +"%Y%m%d")
time=$(date -u +"%Y%m%dT%H%M%SZ")

# Request details
method="GET"
uri="/${bucket}"
query_string="list-type=2"

# Canonical Request
canonical_uri="/${bucket}"
canonical_query_string="${query_string}"
canonical_headers="host:${host}\nx-amz-content-sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\nx-amz-date:${time}\n"
signed_headers="host;x-amz-content-sha256;x-amz-date"
payload_hash="e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855" # Empty payload hash

canonical_request="${method}\n${canonical_uri}\n${canonical_query_string}\n${canonical_headers}\n${signed_headers}\n${payload_hash}"

# String to Sign
algorithm="AWS4-HMAC-SHA256"
scope="${date}/${region}/${service}/aws4_request"
request_hash=$(echo -ne "${canonical_request}" | openssl dgst -sha256 -hex | sed 's/^.* //')
string_to_sign="${algorithm}\n${time}\n${scope}\n${request_hash}"

# Calculate Signature
k_secret="AWS4${secret_key}"
k_date=$(echo -n "${date}" | openssl dgst -sha256 -mac HMAC -macopt hexkey:"$(echo -n "${k_secret}" | xxd -p -c 256)" | sed 's/^.* //')
k_region=$(echo -n "${region}" | openssl dgst -sha256 -mac HMAC -macopt hexkey:"${k_date}" | sed 's/^.* //')
k_service=$(echo -n "${service}" | openssl dgst -sha256 -mac HMAC -macopt hexkey:"${k_region}" | sed 's/^.* //')
k_signing=$(echo -n "aws4_request" | openssl dgst -sha256 -mac HMAC -macopt hexkey:"${k_service}" | sed 's/^.* //')
signature=$(echo -ne "${string_to_sign}" | openssl dgst -sha256 -mac HMAC -macopt hexkey:"${k_signing}" | sed 's/^.* //')

# Authorization Header
authorization="${algorithm} Credential=${access_key}/${scope}, SignedHeaders=${signed_headers}, Signature=${signature}"

# Execute Curl
echo "Inspecting Cloudflare R2 Mirror: ${bucket}..."
echo "Endpoint: ${endpoint}/${bucket}?${query_string}"
echo "----------------------------------------"

response=$(curl -s -G "https://${host}/${bucket}" \
    -H "Authorization: ${authorization}" \
    -H "x-amz-date: ${time}" \
    -H "x-amz-content-sha256: ${payload_hash}" \
    --data-urlencode "list-type=2")

echo "Found Keys (First 50):"
echo "${response}" | grep -oPm1 "(?<=<Key>)[^<]+" | head -n 50
