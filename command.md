<!-- トークンの取得 -->
curl -X POST https://bsky.social/xrpc/com.atproto.server.createSession \
    -H "Content-Type: application/json" \
    -d '{"identifier": { email }, "password": { password }}' | jq

<!-- プロフィールの取得 -->
curl -L -X GET 'https://bsky.social/xrpc/app.bsky.actor.getProfile?actor={ name }.bsky.social' \
-H 'Accept: application/json' \
-H 'Authorization: Bearer { accessToken } | jq
