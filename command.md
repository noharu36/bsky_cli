<!-- トークンの取得 -->
curl -X POST https://bsky.social/xrpc/com.atproto.server.createSession \
    -H "Content-Type: application/json" \
    -d '{"identifier": "noharu0306@gmail.com", "password": "monTarou0306"}' | jq

<!-- プロフィールの取得 -->
curl -L -X GET 'https://bsky.social/xrpc/app.bsky.actor.getProfile?actor={ name }.bsky.social' \
-H 'Accept: application/json' \
-H 'Authorization: Bearer { accessToken } | jq
