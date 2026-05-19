FastComments の LTI 1.3 統合は最小特権の原則に従います: ユーザーを識別し、コメントを正しいコースとリソースに紐づけ、ロールベースの権限を適用するために必要なランチクレームのみを使用します。

このページの残りの部分では、統合が消費するすべてのクレーム、要求しないすべての LTI Advantage サービス、および収集しないすべてのデータカテゴリをマッピングしています。セキュリティおよび調達のレビュアーは、下の表から直接回答を抜き出すことができます。

## LMS から受け取るデータ要素

すべての LTI 1.3 ランチは LMS からの署名付き JWT を伴います。FastComments はその JWT から以下のクレームを抽出し、それ以外は使用しません:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | ランチ間でユーザーを一貫して識別し、同一人物が同じ FastComments SSO ユーザーに解決されるようにする | Yes | Yes, as part of a stable internal SSO ID |
| Display name | `name` | ユーザーのコメントの横に表示される表示名 | Yes (falls back to "LMS User" if absent) | Yes |
| Email | `email` | アカウント照合、通知、モデレーション、サポート対応 | Optional (the integration works without it) | Yes when provided |
| Avatar URL | `picture` | ユーザーのコメントに表示される | Optional | URL only; FastComments does not download or rehost the image |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | ユーザーが管理者、インストラクター（モデレーター）、学習者のいずれかかを判定する | Yes | Derived `isAdmin` / `isModerator` flags on the SSO session |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | コメントスレッドを正しい LMS コースに紐づける | Yes | Yes, as part of the resolved page identifier |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | コース内の正しいアクティビティまたはツール配置にコメントを紐づける | Yes when present | Yes, as part of the resolved page identifier |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | ランチを正しい FastComments テナント設定にルーティングする | Yes | Yes, on the FastComments LTI configuration record |

## 登録時に宣言されるクレームとスコープ

LTI 1.3 のダイナミック登録中に、FastComments は `scope: ""`（追加の OAuth スコープなし）で自身を登録し、次の OpenID Connect クレームのみを宣言します:

`iss`, `sub`, `name`, `email`, `picture`

次の 2 つのメッセージタイプを登録します:

- `LtiResourceLinkRequest` - FastComments への標準的なコースランチ。
- `LtiDeepLinkingRequest` - インストラクターがコース内に FastComments ツールを配置できるようにするもの。

LMS から追加のアクセストークンは要求しません。

## 要求しない LTI Advantage サービス

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | No | 統合はコース名簿を必要としません; ユーザーの身元は各ランチで渡されます |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | No | 統合は成績簿に対応していません |
| Deep Linking beyond the standard placement return | No additional data | ディープリンクはインストラクターによるツール配置のためにのみ使用されます; コースコンテンツの列挙は行いません |

## 収集しないデータ

LTI 自体を超えて、FastComments は LMS またはユーザーから次のものを要求または受信しません:

| Category | Collected? |
|----------|------------|
| Student grades | No |
| Assignment submissions | No |
| Attendance records | No |
| Full course rosters | No |
| Government identifiers | No |
| Date of birth | No |
| Postal address or phone number | No |
| Financial information | No |
| LMS administrator credentials | No |

## アクセスの境界

- FastComments が受け取るデータは、LMS の登録鍵で署名された許可された LTI 1.3 ランチ内のデータのみです。統合は追加情報のために LMS にコールバックしません。
- ランチトークンは一度きりで短期間有効です。再生されたトークンや有効期限切れのトークンは拒否されます。
- LMS 管理者はツールがプラットフォーム内のどこに配置されるかを制御します。例えば D2L Brightspace はデプロイごとの org-unit スコーピングとデプロイごとのセキュリティ設定をサポートしており、管理者はツールをグローバルに利用可能にするのではなく、特定のコースや組織単位に限定できます。Moodle、Blackboard、Sakai、および Schoology もそれぞれの LTI 1.3 実装で同等のデプロイごとの制御を提供します。

## 保存と保持

FastComments はアクティブなコメントサービスの期間中および顧客が設定した保持設定に従って LTI 派生データを保持します。コメントデータは保存時に暗号化された本番ストレージに保存されます。アカウント終了または書面による削除要求があった場合、FastComments は適用される契約に従って顧客データを削除または匿名化します。

完全な保存およびデータ取り扱いの詳細については、<a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments プライバシーポリシー</a> を参照してください。

## レビューの頻度

追加のクレーム、スコープ、または LTI Advantage サービスを必要とする新しい LTI 機能は、要求されたアクセスが提供される機能に対して必要かつ比例的であることを確認するためにリリース前にレビューされます。

## セキュリティ質問票向けの短い声明

> FastComments は LTI 1.3 統合において最小特権とデータ最小化を適用します。統合はユーザーを認証するために必要な LTI ランチクレーム（`sub`, `name`, `email`, `picture`）、ユーザーの役割を判断するための情報、そしてコメントが属するコースとリソースを特定するための情報のみを使用します。FastComments は Names and Role Provisioning Services、Assignment and Grade Services、成績データ、出席情報、フルロスター、または LMS 管理者アクセスを要求しません。LMS 管理者はツールが利用可能な組織単位、コース、およびデプロイを引き続き制御します。