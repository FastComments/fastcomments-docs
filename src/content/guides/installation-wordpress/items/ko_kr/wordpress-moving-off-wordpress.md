---
If you are moving your site off of WordPress and want FastComments on the new site, you do not need the WordPress plugin. Export your comments from WordPress, then upload the file on the [가져오기 페이지](https://fastcomments.com/auth/my-account/manage-data/import) in the FastComments dashboard.

We support two WordPress export formats.

### WordPress XML (권장)

This is the file from WordPress's built-in exporter, so no extra plugin is needed.

1. In your WordPress admin, go to `Tools -> Export`.  
   WordPress 관리 화면에서 `Tools -> Export` 로 이동합니다.
2. Select `All content` and click `Download Export File`.  
   `All content` 를 선택하고 `Download Export File` 을 클릭합니다.
3. On the FastComments [가져오기 페이지](https://fastcomments.com/auth/my-account/manage-data/import), select `WordPress (.xml)` and upload the file.  
   FastComments [가져오기 페이지](https://fastcomments.com/auth/my-account/manage-data/import)에서 `WordPress (.xml)` 를 선택하고 파일을 업로드합니다.

Each comment is tied to the URL of the post it was left on, which is already in the file.  
각 댓글은 남겨진 게시물의 URL에 연결되어 있으며, 이는 파일에 이미 포함되어 있습니다.

The import keeps the author name, email, and website, the date, the content, reply threading, and whether the comment was approved. Commenter avatars are brought over from Gravatar. Votes are not part of this format.  
가져오기는 작성자 이름, 이메일, 웹사이트, 날짜, 내용, 답글 스레드 및 댓글 승인 여부를 유지합니다. 댓글 작성자 아바타는 Gravatar에서 가져옵니다. 투표는 이 형식에 포함되지 않습니다.

### WordPress CSV

This is the file from [WebToffee's WordPress Comments Import & Export plugin](https://wordpress.org/plugins/comments-import-export-woocommerce/).  
이 파일은 [WebToffee의 WordPress Comments Import & Export 플러그인](https://wordpress.org/plugins/comments-import-export-woocommerce/)에서 가져온 것입니다.

1. Install the plugin in your WordPress admin and export your comments as CSV.  
   WordPress 관리 화면에 플러그인을 설치하고 댓글을 CSV 형식으로 내보냅니다.
2. Replace each `comment_post_ID` value with the post's URL.  
   각 `comment_post_ID` 값을 해당 게시물의 URL로 교체합니다.
3. On the FastComments [가져오기 페이지](https://fastcomments.com/auth/my-account/manage-data/import), select `WordPress (.csv)` and upload the file.  
   FastComments [가져오기 페이지](https://fastcomments.com/auth/my-account/manage-data/import)에서 `WordPress (.csv)` 를 선택하고 파일을 업로드합니다.

Each comment is tied to the `comment_post_ID` column. WordPress fills this column with the post ID, and your new site does not have WordPress post IDs, so step 2 replaces it with the URL.  
각 댓글은 `comment_post_ID` 열에 연결됩니다. WordPress는 이 열에 게시물 ID를 채우지만, 새 사이트에는 WordPress 게시물 ID가 없으므로 2단계에서 이를 URL로 교체합니다.

The import keeps the author name, email, and website, the date, the content, reply threading, and whether the comment was approved. Commenter avatars are brought over from Gravatar. It also keeps WordPress's spam flag, and wpDiscuz likes and dislikes when the file includes them.  
가져오기는 작성자 이름, 이메일, 웹사이트, 날짜, 내용, 답글 스레드 및 댓글 승인 여부를 유지합니다. 댓글 작성자 아바타는 Gravatar에서 가져옵니다. 또한 파일에 포함된 경우 WordPress의 스팸 플래그와 wpDiscuz의 좋아요·싫어요도 유지합니다.

### 새 페이지에 댓글 매칭하기

If your new site keeps the same URLs as your WordPress site, the comments show up on the matching pages with no extra setup.  
새 사이트가 WordPress 사이트와 동일한 URL을 유지한다면, 별도의 설정 없이도 해당 페이지에 댓글이 표시됩니다.

If the domain changes, run the [Domain Migration tool](/guide-migrations.html#migrating-domains) after the import. If individual page URLs change, you can [migrate each page](/guide-migrations.html#migrating-pages) from its old URL to the new one.  
도메인이 변경된 경우, 가져온 후 [도메인 마이그레이션 도구](/guide-migrations.html#migrating-domains)를 실행하세요. 개별 페이지 URL이 변경되면, 기존 URL에서 새 URL로 [각 페이지를 마이그레이션](/guide-migrations.html#migrating-pages)할 수 있습니다.

For bulk page migrations, such as removing the domain from the value you pass to the comment widget's [urlId](/guide-customizations-and-configuration.html#url-id) field, [open a support ticket](https://fastcomments.com/auth/my-account/help) and we will handle it for you.  
댓글 위젯의 [urlId](/guide-customizations-and-configuration.html#url-id) 필드에 전달하는 값에서 도메인을 제거하는 등 대량 페이지 마이그레이션이 필요할 경우, [지원 티켓을 열어](https://fastcomments.com/auth/my-account/help) 주세요. 저희가 처리해 드립니다.

### 전환하기 전에

You can run the import as many times as you like. Re-importing the same file [does not create duplicates](/guide-migrations.html#importing-data), so you can import once to test the new site, then import again with your latest comments right before switching over.  
원하는 만큼 가져오기를 실행할 수 있습니다. 동일한 파일을 다시 가져와도 [중복이 생성되지 않으며](/guide-migrations.html#importing-data), 새 사이트를 테스트하기 위해 한 번 가져온 뒤, 전환 직전에 최신 댓글로 다시 가져올 수 있습니다.

For export files larger than 1GB, [reach out to support](https://fastcomments.com/auth/my-account/help).  
1GB보다 큰 내보내기 파일의 경우, [지원팀에 문의하세요](https://fastcomments.com/auth/my-account/help).

To add FastComments to your new site, see the [Installation guide](/guide-installation.html).  
새 사이트에 FastComments를 추가하려면, [설치 가이드](/guide-installation.html)를 참고하세요.