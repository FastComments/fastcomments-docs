White Labelingのセットアップ手順は以下の通りです:

1. 請求の処理方法を選択します。
   1. FastComments Proでは、一定数のホワイトラベルテナントまで固定額を支払います。
   2. FastComments Flexでは、テナントごと、およびそのテナントの利用量に応じて支払います。
   3. どちらの場合も、各テナントの上限は自分で設定します。
      1. 上限はテナントごとにカスタマイズできます。さらに、販売するパッケージを更新する場合も、既存の顧客に提供済みの価格設定に影響を与えることなく行えます。
2. FastComments.comの用語に慣れましょう:
   1. `Tenant`は「顧客」です。
   2. `TenantUser`は、`Tenant`内で一連の権限を持つ管理者です。
   3. `TenantPackage`は、`Tenant`が利用可能な上限と価格のセットを持つパッケージです。
3. [API](/guide-api.html)と統合するか、[スクリプト](https://github.com/FastComments/fastcomments-code-examples/tree/master/white-labeling)を使用してテナントをオンボーディングします。