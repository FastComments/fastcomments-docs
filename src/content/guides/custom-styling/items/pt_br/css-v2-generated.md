[inline-code-attrs-start title = 'CSS Padrão do Widget de Comentários'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
body { margin: 0; padding: 0; }
    d { display: block }
    @keyframes pop-in { 0% { transform: scale(0.3); } 50% { transform: scale(1.1); } 100% { transform: scale(1); } }
    @keyframes placeHolderShimmer { 0% { background-position: 0 0 } 100% { background-position: 100% 0 } }
    .comment-reply .comment-input.animated-background, .comment-reply input.animated-background, .animated-background { animation-duration: 1s; animation-iteration-count: infinite; animation-name: placeHolderShimmer; background: #f6f7f8; background: linear-gradient(to right, #eee 8%, #dddddd 18%, #eee 33%); background-size: 50% 50%; }
    @keyframes backgroundToNormal { from { background: lightblue } to { background: inherit } }
    .fast-comments { position: relative; width: 100%; font-size: 13px }
    .hidden { display: none }
    .invisible { visibility: hidden }
    .inline-block { display: inline-block }
    .icon { display: inline-block; width: 24px; height: 24px; vertical-align: middle; image-rendering: -webkit-optimize-contrast; }
    .icon.bubble { background: url("${FC_CDN}/images/svg/v2/text_bubble_dark.svg") no-repeat center; background-size: 22px 22px; }
    .icon.bubble-white { background: url("${FC_CDN}/images/svg/v2/text_bubble_white.svg") no-repeat center; background-size: 22px 22px; }
    .icon.cross { background: url("${FC_CDN}/images/svg/v2/close.svg") no-repeat center; background-size: 9px 9px; }
    .icon.reply-arrow-inactive { background: url("${FC_CDN}/images/svg/v2/reply_inactive.svg") no-repeat center; background-size: 15px 15px; }
    .icon.reply-arrow-active { background: url("${FC_CDN}/images/svg/v2/reply.svg") no-repeat center; background-size: 15px 15px; }
    .icon.up { background: url("${FC_CDN}/images/svg/v2/thumbs_up_light.svg") no-repeat center; background-size: 12px 12px; }
    .icon.up.active, .icon.up:hover { background: url("${FC_CDN}/images/svg/v2/thumbs_up_dark.svg") no-repeat center; background-size: 12px 12px; }
    .icon.down { background: url("${FC_CDN}/images/svg/v2/thumbs_down_light.svg") no-repeat center; background-size: 12px 12px; }
    .icon.down.active, .icon.down:hover { background: url("${FC_CDN}/images/svg/v2/thumbs_down_dark.svg") no-repeat center; background-size: 12px 12px; }
    .icon.pin-small { background: url("${FC_CDN}/images/svg/v2/pin.svg") no-repeat center; background-size: 15px 15px; }
    .icon.logo { width: 27px; height: 33px; background: url("${FC_CDN}/images/svg/v2/logo.svg") no-repeat center; background-size: 22px;  }
    .icon.edit-small { background: url("${FC_CDN}/images/svg/menu.svg") no-repeat center; background-size: 17px 17px; }
    .icon.edit-big { background: url("${FC_CDN}/images/svg/v2/edit.svg") no-repeat center; background-size: 22px 22px; }
    .icon.trash { background: url("${FC_CDN}/images/svg/v2/trash_thin.svg") no-repeat center; background-size: 22px 22px; }
    .icon.eye { background: url("${FC_CDN}/images/svg/v2/view.svg") no-repeat center; background-size: 22px 22px; }
    .icon.eye-slash { background: url("${FC_CDN}/images/svg/v2/view_hide.svg") no-repeat center; background-size: 22px 22px; }
    .icon.replied { background: url("${FC_CDN}/images/svg/v2/replied.svg") no-repeat center; background-size: 22px 22px; }
    .icon.bold { background: url("${FC_CDN}/images/svg/v2/editor_bold.svg") no-repeat center; background-size: 9px; }
    .icon.ul { position: relative; top: 1px; background: url("${FC_CDN}/images/svg/v2/editor_underline.svg") no-repeat center; background-size: 10px; }
    .icon.it { background: url("${FC_CDN}/images/svg/v2/editor_itallic.svg") no-repeat center; background-size: 7px; }
    .icon.s { background: url("${FC_CDN}/images/svg/v2/editor_strike.svg") no-repeat center; background-size: 10px; }
    .icon.code { background: url("${FC_CDN}/images/svg/v2/editor_embed.svg") no-repeat center; background-size: 16px; }
    .icon.link { position: relative; top: -1px; background: url("${FC_CDN}/images/svg/v2/editor_link.svg") no-repeat center; background-size: 14px; }
    .icon.img-up { background: url("${FC_CDN}/images/svg/v2/editor_image.svg") no-repeat center; background-size: 16px; }
    .icon.img-btn-wrap { position: relative; }
    .icon.return { background: url("${FC_CDN}/images/svg/return.svg") no-repeat center; background-size: 22px; margin-left: 6px; }
    .icon.gif { width: auto; height: auto; font-size: 0; }
    .icon.gif::before { content: "GIF"; font-size: 12px; }
    .icon.gif::before, .t-btn.txt { display: inline-block; width: auto; height: auto; margin-right: 3px; text-align: center; vertical-align: middle; font-weight: 500; }
    .icon.bell { background: url("${FC_CDN}/images/svg/v2/bell.svg") no-repeat center; background-size: 22px; }
    .icon.bell-red { background: url("${FC_CDN}/images/svg/v2/bell-red.svg") no-repeat center; background-size: 22px; }
    .divider { display: inline-block; height: 25px; margin: 0 10px; vertical-align: middle; border-right: 1px solid #c2c2c2; }
    .icon.block { background: url("${FC_CDN}/images/svg/v2/ban.svg") no-repeat; background-size: 22px 22px; }
    .icon.flag { background: url("${FC_CDN}/images/svg/flag.svg") no-repeat; background-size: 22px 22px; }
    .icon.flag-small { background: url("${FC_CDN}/images/svg/flag.svg") no-repeat center; background-size: 15px 15px; }
    .select-dir-wrapper { clear: both; text-align: right; border-bottom: 1px solid #afafaf; }
    .select-dir-wrapper .comment-count { float: left; font-weight: 500; }
    .select-dir-wrapper > * { margin: 9px; height: 16px }
    .select-dir { display: inline-block }
    .dropdown { position: relative; z-index: 3; text-align: right }
    .dropdown .drop-label { padding: 0 0 7px 0; cursor: pointer; font-weight: 500 }
    .dropdown .drop-label i { position: relative; display: inline-block; top: -2px; font-size: 7px }
    .dropdown .items { position: absolute; display: none; top: 20px; right: 0; width: 150px; overflow: hidden; border-radius: 0 0 4px 4px; background: #fff; }
    .dropdown:hover .items { position: absolute; display: block }
    .dropdown .items > * { padding: 5px 9px; cursor: pointer; font-weight: 500; text-align: left; font-size: 13px }
    .no-comments { clear: both; text-align: center; font-weight: 500; font-size: 16px; }
    .new-comments-message { width: fit-content; margin: 20px auto 0; padding: 5px 10px; text-align: center; cursor: pointer; font-weight: 500 }
    .new-comments-message .new-comments-count { pointer-events: none; position: relative; top: 1px; display: inline-block; min-width: 12px; padding: 2px 5px 4px 5px; margin-right: 3px; border: 1px solid #a2a2a2; border-radius: 4px 0 4px 4px; }
    .new-comments-message span { pointer-events: none; padding-bottom: 2px; border-bottom: 1px solid #a3a3a3; }
    .comment .new-comments-message { margin: 10px auto 0; }
    .sso-login-wrapper, .fastcomments-message-wrapper { display: flex; height: fit-content; min-height: 130px; padding: 30px 0; box-sizing: border-box; align-items: center; justify-content: center; border: 1px solid #bfbfbf; border-radius: 0 11px 11px 11px; }
    .sso-login-wrapper .message-text, .fastcomments-message-wrapper .message-text { display: inline; pointer-events: none; }
    .fastcomments-message-wrapper .message-text a { color: #fff; pointer-events: all; }
    .sso-login-wrapper .sso-login, .fastcomments-message-wrapper .fastcomments-message { display: inline-block; animation: pop-in 0.5s; animation-timing-function: ease; padding: 10px 17px 10px 27px; border-radius: 0 7px 7px 7px; background: #333; color: #fff; text-decoration: none; font-size: 17px; font-weight: 500; }
    .fastcomments-message-wrapper .fastcomments-message { margin: 0 5%; cursor: default; }
    .sso-login-wrapper .sso-login[href] { cursor: pointer; }
    .sso-login-wrapper .sso-login .message-text, .fastcomments-message-wrapper .fastcomments-message .message-text { margin-right: 10px; }
    .sso-login-wrapper .sso-login .icon, .fastcomments-message-wrapper .fastcomments-message .icon { pointer-events: none; }
    .sso-login-wrapper .sso-login.clickable { cursor: pointer; }
    .comment .sso-login-wrapper { padding: 7px 0; text-align: left }
    .comment .sso-login-wrapper .sso-login { font-size: 16px }
    .default-hidden { transition-duration: 300ms }
    .comment-input:not(.show-default-hidden) .default-hidden { height: 0; margin: 0 !important; opacity: 0; pointer-events: none; transition-duration: 200ms }
    .loading .pagination { opacity: 0.5; pointer-events: none; }
    .card { border: 1px solid #d0d0d0; background: #fdfdfd; border-radius: 3px; box-shadow: 5px 5px 7px rgba(0,0,0,.1) }
    button, .button { display: inline-block; margin-bottom: 10px; padding: 4px 10px; border-radius: 0 7px 7px 7px; font-size: 15px; background: #fbfbfb; color: #333; text-decoration: none; border: 1px solid #a2a2a2; cursor: pointer }
    .fast-comments, textarea { font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif }
    textarea::placeholder { font-size: 13px; font-weight: 500; transition-duration: 150ms }
    textarea:focus::placeholder { color: transparent }
    .fc-red { display: inline-block; margin: 5px; color: #ff0000 }
    .comment-error { padding: 10px }
    input, textarea { padding: 12px 20px; border: 1px solid #bfbfbf; border-radius: 0 11px 11px 11px; box-sizing: border-box; outline: none; background: transparent; }
    input { padding: 9px 12px; border-radius: 0 6px 6px 6px; }
    input:focus, textarea:focus, textarea:focus + .horizontal-border-wrapper, .comment-input textarea:focus { border-color: #555 } /* .comment-reply textarea:focus para SSR */
    .pagination { margin-top: 50px; line-height: 19px; text-align: center; user-select: none; }
    .pagination > * { display: inline-block; cursor: pointer; font-weight: 700; }
    .pagination > * > span { font-weight: normal; pointer-events: none; }
    .pagination > * > span::before { content: "("; }
    .pagination > * > span::after { content: ")"; }
    .pagination .load-next-page { padding-right: 25px; }
    .pagination .load-all { padding-left: 25px; border-left: 2px solid #555; } /* é importante que a borda esteja no item à direita, pois nem sempre aparece */
    .comments-toggle { display: block; width: fit-content; margin: 20px auto; cursor: pointer; padding: 10px 17px 10px 27px; border-radius: 7px; background: #333; color: #fff; text-decoration: none; font-size: 17px; font-weight: 500; user-select: none; }
    .replying-to { margin-bottom: 5px }
    .comment-reply { position: relative; width: 100%; margin: 10px 0 15px 0; }
    .comment-reply.root { padding: 15px; box-sizing: border-box; }
    @media(max-width: 500px) { .comment-reply.root { padding: 15px 5px 15px 5px; } }
    .comment-reply .fast-comments-waiting { display: block; margin: 5px 0 }
    .comment-reply .comment-reply-top-bar { position: relative; min-height: 25px; margin: 0 26px 15px 26px; line-height: 25px; } /* posicionado como 'relative' para itens como a lista de notificações */
    .comment-reply .comment-reply-top-bar .logged-in-info { display: inline-block; width: calc(100% - 60px); min-width: 150px; }
    .comment-reply .comment-reply-top-bar .logged-in-info .avatar { display: inline-block; height: 25px; vertical-align: middle; margin-right: 5px; border-radius: 25px; overflow: hidden; box-shadow: 3px 3px 3px 0 rgba(0, 0, 0, 0.07); font-size: 0; }
    .comment-reply .comment-reply-top-bar .logged-in-info .avatar .open-profile { cursor: pointer; }
    .comment-reply .comment-reply-top-bar .logged-in-info .avatar.animated-background img { opacity: 0.1; }
    .comment-reply .comment-reply-top-bar .logged-in-info .avatar img { width: 25px; height: 25px; object-fit: cover; }
    .comment-reply .comment-reply-top-bar .logged-in-info .username { display: inline-block; max-width: calc(50% - 25px); overflow: hidden; vertical-align: middle; text-overflow: ellipsis; font-weight: 700; white-space: nowrap; }
    .comment-reply .comment-reply-top-bar .logged-in-info .badges { display: inline-block; margin-left: 5px; }
    .comment-reply .comment-reply-top-bar .right { float: right; }
    .comment-reply .comment-reply-top-bar .right > * { display: inline-block; }
    .comment-reply .comment-reply-top-bar .right .menu { font-weight: 500; font-size: 11px; }
    .comment-reply .comment-reply-top-bar .right .menu:hover { z-index: 9002; }
    .comment-reply .comment-reply-top-bar .right .menu .drop-label i { display: inline-block; width: 4px; height: 4px; background: #333; border-radius: 4px; margin: 0 2px; }
    .comment-reply .comment-reply-top-bar .right .menu .items { top: 25px; padding: 20px; box-shadow: 2px 3px 6px rgba(0, 0, 0, 0.1); border-radius: 10px 0 10px 10px; }
    .comment-reply .comment-reply-top-bar .right .menu .items > * { font-weight: 700; }
    .comment-reply .comment-reply-top-bar .right .notification-bell { position: relative; margin-left: 5px; cursor: pointer; }
    .comment-reply .comment-reply-top-bar .right .notification-bell .count { position: absolute; top: -3px; left: 19px; min-width: 15px; height: 15px; text-align: center; font-size: 11px; pointer-events: none; color: red; display: none; }
    .comment-reply .comment-reply-top-bar .right .notification-bell .icon.bell-red { display: none; }
    .comment-reply .comment-reply-top-bar .right .notification-bell > * { pointer-events: none; }
    .comment-reply .comment-reply-top-bar .right .notification-bell.has-notifications .icon.bell { display: none; }
    .comment-reply .comment-reply-top-bar .right .notification-bell.has-notifications .icon.bell-red { display: inline-block; }
    .comment-reply .comment-reply-top-bar .right .notification-bell.has-notifications .count { display: block; }
    .toolbar { position: relative; display: inline-block; margin: 5px 0 0 0; font-size: 13px }
    .toolbar .t-btn { display: inline-block; margin: 0 1px; vertical-align: middle; cursor: pointer; transition-duration: 200ms; user-select: none; }
    .toolbar .t-btn:hover { opacity: 0.7; }
    .toolbar .img-btn-wrap { display: inline-block; overflow: hidden }
    .toolbar .t-btn input[type=file] { position: absolute; padding: 40px; font-size: 100px; top: 0; left: 0; opacity: 0; cursor: pointer }
    .commenty-input:not(.show-default-hidden) .toolbar { width: 100%; margin-top: -50px; }
    @media(max-width: 500px) { .toolbar { display: flex; width: 100%; padding-bottom: 12px; justify-content: space-evenly; } }
    @media(max-width: 500px) { .comment-input .toolbar { margin-top: 50px; border-bottom: 1px solid #bfbfbf; } }
    .comment-reply .auth-input, .comment-vote-auth.auth-input { margin: 10px 0; font-size: 13px }
    .comment-reply .auth-input .fc-login { margin: 10px 0 0; }
    .comment-reply .auth-input .reasoning, .comment-vote-auth.auth-input .reasoning { font-weight: 600; }
    .comment-reply .auth-input .fc-red, .comment-vote-auth.auth-input .fc-red { display: block }
    .comment-reply .auth-input input, .comment-vote-auth.auth-input input { width: 100%; margin-top: 10px; padding: 9px 12px; border-radius: 0 6px 6px 6px; font-size: 14px; border: 1px solid #a2a2a2; }
    .comment-reply .auth-input .solicitation-info, .comment-vote-auth.auth-input .solicitation-info { margin-top: 10px; }
    .comment-reply .auth-input .fast-comments-reply { margin-top: 10px; padding: 10px 45px; border-radius: 5px 0 5px 5px; background: #333; color: #fff; border: none; }
    .comment-input, .comment-edit { position: relative; padding-bottom: 30px; border-radius: 0 11px 11px 11px; }
    .comment-input textarea { display: block; width: 100%; height: 130px; padding: 15px 25px 15px 15px; resize: none; font-size: 16px; border-bottom: none; border-radius: 0 11px 0 0; }
    .comment-input textarea::placeholder { font-size: 16px; font-weight: 400; }
    .comment-input input { display: block; width: 100%; font-size: 14px; }
    .comment-input .fastcomments-message-wrapper { border: 1px solid #bfbfbf; border-bottom: none; border-radius: 0 11px 0 0; }
    .comment-input input[name=fastcomments-link] { display: block; width: 100%; margin: 10px 0; }
    .comment-input .horizontal-border-wrapper { pointer-events: none; border-color: #bfbfbf; }
    .comment-input .horizontal-border { position: absolute; height: 20px; border-bottom: 1px solid; border-color: inherit; }
    .comment-input .horizontal-border-left { bottom: 0; left: 0; border-radius: 0 0 0 11px; }
    .comment-input .horizontal-border-right { bottom: 0; right: 0; width: 20px; border-radius: 0 0 11px 0; }
    .comment-input .horizontal-border-top-left, .comment-input .horizontal-border-top-right { display: none; position: absolute; top: 20px; border-bottom: 0; border-top: 1px solid; border-color: inherit; }
    .comment-input .horizontal-border-top-right { top: 0; right: 0; border-radius: 0 11px 0 0; width: 20px; }
    .comment-input .horizontal-border-top-left { top: 0; left: 0; }
    .comment-input .horizontal-border-bottom-left { position: absolute; height: 30px; width: 15px; left: 0; bottom: 0; border-color: inherit; border-left-width: 1px; border-left-style: solid; border-radius: 0 0 0 11px; }
    .comment-input .horizontal-border-bottom-right { position: absolute; height: 30px; width: 15px; right: 0; bottom: 0; border-color: inherit; border-right-width: 1px; border-right-style: solid; border-radius: 0 0 11px 0; }
    .comment .comment-input .horizontal-border-top-left, .comment .comment-input .horizontal-border-top-right { display: block }
    @media(max-width: 500px) { .comment-input textarea, .comment-input .fastcomments-message-wrapper { height: 130px;  } }
    .comment .reply-button-wrapper { position: relative; float: right; top: -19px; right: 26px; border-radius: 0 7px 7px 7px; }
    .comment .fast-comments-reply, .comment .edit-save { padding: 10px 27px; border-radius: 0 7px 7px 7px; background: #333; color: #fff; text-decoration: none; }
    .comment .cancel-button-wrapper { position: absolute; top: 9px; right: 25px; border-radius: 4px; }
    .comment .comment-edit .cancel-button-wrapper { top: -13px }
    .comment .fast-comments-reply-cancel { margin-bottom: 0; padding: 1px 1px; border-radius: 4px; }
    .comment .fast-comments-reply-cancel .cross { pointer-events: none; }
    .comment-reply.root .reply-button-wrapper { position: relative; float: right; top: -20px; right: 27px; margin-bottom: 10px; border-radius: 0 7px 7px 7px; }
    .comment-reply.root button { margin-bottom: 0; padding: 7px 20px; font-weight: 600; }
    .comment-reply.root button .bubble { margin-left: 10px; pointer-events: none; }
    .comments { clear: both; padding: 15px 0; }
    @media(max-width: 500px) { .comments { padding: 15px 5px; } }
    .comment { position: relative; margin-top: 15px }
    @media(max-width: 500px) { .comment { margin-top: 5px; } }
    .comment .comment-text spoiler:not(:hover) { background: #eee; color: #eee; border: 1px dotted #a2a2a2; }
    .comment .comment-text .inline-image { display: block; max-width: 500px; margin: 3px 0 3px 0 } /* não deve selecionar 'inline-image' no WYSIWYG */
    .comment .comment-text .inline-image img { max-width: 100%; max-height: 400px } /* não deve selecionar 'inline-image' no WYSIWYG */
    .disable-image-redirect .comment .inline-image { cursor: default; }
    .comment.is-live > .inner { animation: backgroundToNormal 1.5s }
    .comment.is-unread > .inner .avatar-wrapper { box-shadow: 0 0 20px #89f796; }
    .comment.unverified > .commenter-name, .comment.unverified > .comment-content .commenter-name { opacity: 0.7 }
    .comment.unverified > .comment-content > .comment-text { opacity: 0.7 }
    .comment.is-spam > .comment-content { border: 1px solid red; } /* pode ser definido ao salvar o comentário inicialmente */
    .comment.is-unapproved > .comment-content { border: 1px solid #ffa700; } /* pode ser definido ao salvar o comentário inicialmente */
    .comment.is-blocked > .inner > .comment-content > .comment-text { max-height: 40px; overflow: hidden; }
    .comment > .requires-verification-approval { margin: 3px 0 6px }
    .comment > .inner > .spam-notice { margin: 0 0 10px 0; font-size: 12px; color: red }
    @media(max-width: 500px) { .comment > .inner > .spam-notice { margin-top: 18px; } }
    .comment .avatar-wrapper { position: relative; display: inline-block; width: 56px; height: 56px; overflow: hidden; box-shadow: 3px 3px 5px 0 rgba(0, 0, 0, 0.10); border-radius: 15px 0 15px 15px; vertical-align: top; }
    .comment .avatar-wrapper.anon { border: 1px solid #3f3f3f; }
    .comment .avatar-wrapper .open-profile { cursor: pointer; }
    .comment .avatar-wrapper.animated-background img { opacity: 0.1; }
    @media(max-width: 500px) { .comment .avatar-wrapper { width: 36px; height: 36px; } }
    .comment .avatar { width: 100%; height: 100%; object-fit: cover; }
    @media(max-width: 500px) { .comment .avatar { vertical-align: middle } }
    .comment .commenter-name .badges { margin-bottom: 5px; }
    .badges .badge { display: inline-block; vertical-align: middle; line-height: initial; margin: 3px 5px 3px 0; padding: 5px 7px; cursor: default; font-size: 12px; white-space: nowrap; border-radius: 4px; color: #000; }
    .badges .badge img { max-width: 22px; }
    .badges .badge.ib { padding: 0; }
    .comment .commenter-name { font-size: 14px }
    .comment > .commenter-name { display: none; vertical-align: middle; color: #000 }
    @media(max-width: 500px) { .comment > .commenter-name { display: inline-block; margin-left: 5px; vertical-align: middle } }
    .comment > .commenter-name a { display: block; color: #000; text-decoration: none }
    .comment .commenter-name .website-url { color: #000; text-decoration: underline }
    .comment .commenter-name .label { font-size: 10px; text-transform: uppercase; font-weight: 500; color: #666666; } /* common label styling (unverified label, admin label, custom labels, etc) */
    .comment > .inner { position: relative; padding: 8px 8px 10px 8px; } /* 'relative' é necessário para mensagem bloqueada */
    @media(max-width: 500px) { .comment > .inner { padding: 5px 5px 5px 5px; } }
    .comment > .inner > .comment-content { position: relative; display: inline-block; width: calc(99% - 101px); margin-left: 15px; }
    .hide-avatars .comment > .inner > .comment-content { margin-left: 0; }
    .hide-avatars .comments > .comment > .inner > .comment-content > .commenter-name, .hide-avatars .comments > .comment > .inner > .comment-content > .comment-text { padding-left: 0; }
    @media(max-width: 500px) { .comment > .inner > .comment-content { display: inline; margin-left: 3px; background: transparent } }
    .comment > .inner > .comment-content .commenter-name { display: inline-block; max-width: 50%; vertical-align: middle; padding: 5px 0 0 2px; color: #171717 }
    .comment > .inner > .comment-content .commenter-name > .username { display: block; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
    @media(max-width: 500px) { .comment > .inner > .comment-content .commenter-name { padding: 5px 0 0 4px; } }
    .comment > .inner > .comment-content .comment-text { padding: 5px 2px; color: #000; line-height: 22px; font-size: 14px; word-break: break-word; white-space: pre-line; overflow-y: auto }
    .comment > .inner > .comment-content .comment-text ol, .comment > .inner > .comment-content .comment-text ul { white-space: normal; }
    .comment > .inner > .comment-content .comment-text .react { display: inline; max-height: 20px; margin: 0 3px; vertical-align: text-top; }
    .comment > .inner > .comment-content > .comment-text b > a { color: #000; text-decoration: none; }
    .comment > .inner > .comment-content > .comment-text blockquote { margin: 15px 0; padding: 0 20px; border-left: 1px solid #e5e5e5; }
    .comment > .inner > .comment-content .comment-text-edit { width: 100%; margin-top: 15px; }
    @media(max-width: 500px) { .comment > .children .comment-text { margin-left: 27px; } }
    .comment > .inner > .comment-content .comment-text br { line-height: 0.5em }
    .comment > .inner > .comment-content textarea.comment-text, .comment > .inner > .comment-content input.comment-text { display: block; width: calc(100% - 20px); height: fit-content; margin: 10px 10px 0 10px; padding: 5px 11px; resize: vertical }
    .comment > .inner > .comment-content .edit-failure { display: block; margin: 10px 0; text-align: center }
    .comment > .inner > .comment-content .comment-toolbar-vote { position: relative; margin-left: 8px }
    @media(max-width: 500px) { .comment > .inner > .comment-content .comment-toolbar-vote { margin-left: 0 } }
    .comment > .inner > .comment-content .comment-toolbar-vote .vote-awaiting-verification { padding: 5px 0; font-weight: 500 }
    .comment > .inner > .comment-bottom { margin-top: 25px; border-bottom: 1px solid #e5e5e5; }
    @media(max-width: 500px) { .comment > .inner > .comment-bottom { margin-top: 10px; } }
    .children .comment > .inner > .comment-bottom { margin-left: 28px; }
    @media(max-width: 500px) { .children .comment > .inner > .comment-bottom { margin-left: 21px; } }
    @media(min-width: 500px) { .hide-avatars .children .comment > .inner > .comment-bottom { margin-left: 33px; } }
    .comment > .inner > .comment-bottom > .comment-bottom-toolbar { position: relative; min-height: 36px; }
    .comment > .inner > .comment-bottom .comment-vote-options .votes-up, .comment > .inner > .comment-bottom .comment-vote-options .votes-down { position: relative; top: 1px; vertical-align: middle; font-size: 12px; font-weight: 500; }
    .comment > .inner > .comment-bottom .comment-vote-options .votes-up { margin-right: 5px; }
    .comment > .inner > .comment-bottom .comment-vote-options .votes-down { margin-left: 5px; }
    .comment > .inner > .comment-bottom .comment-toolbar-vote .comment-votes .divider { height: 20px; }
    .comment > .inner > .comment-bottom .comment-vote-options { display: inline-block; margin: 0 7px 0 2px; font-size: 12px; }
    .comment > .inner > .comment-bottom .comment-vote-options > span { cursor: pointer }
    .comment > .inner > .comment-bottom .comment-votes, .comment > .inner > .comment-bottom .comment-reply-start { display: inline-block; margin: 5px 2px; font-size: 12px }
    .comment > .inner > .comment-bottom .comment-vote-options button { margin: 0 3px }
    .comment > .inner > .comment-bottom .comment-vote-options .view-count { display: inline-flex; align-items: center; vertical-align: middle; gap: 3px; margin-left: 3px; color: #828282; }
    .comment > .inner > .comment-bottom .comment-vote-options .view-count .icon { opacity: 0.7; }
    .comment > .inner > .comment-bottom .comment-toolbar-reply { position: absolute; right: 0; top: -4px; }
    .comment > .inner > .comment-bottom .comment-reply-start { border: none; background: none; font-weight: 600; }
    .comment > .inner > .comment-bottom .comment-reply-start i { position: relative; top: -2px; pointer-events: none; }
    .comment > .inner > .comment-bottom .comment-vote-auth { width: 100%; max-width: 400px }
    .comment > .inner > .comment-bottom .comment-vote-auth .fast-comments-waiting { float: left }
    .comment > .inner > .comment-bottom .comment-vote-auth .buttons { text-align: right }
    .comment > .inner > .comment-bottom .comment-vote-auth button { margin: 5px 0 0 5px; padding: 10px 35px; background: #333; color: #fff; border: none; }
    .comment > .inner > .comment-bottom .reply-form-wrapper { padding: 5px }
    .comment > .toggle-replies { margin: 0 0 0 15px; line-height: 24px; font-weight: 500; font-size: 11px; color: #666; cursor: pointer; user-select: none; }
    .children .comment > .toggle-replies { margin-left: 41px; }
    @media(max-width: 500px) { .children .comment > .toggle-replies { margin-left: 12px; } }
    .comment > .toggle-replies > * { vertical-align: middle; pointer-events: none; }
    .comment > .toggle-replies > i { margin-right: 5px; }
    .comment > .toggle-replies > span > .count { color: #1f1f1f; }
    .comment > .toggle-replies > span > .count:before { content: "("; }
    .comment > .toggle-replies > span > .count:after { content: ")"; }
    .comment .prompt { position: absolute; top: -2px; left: 0; width: 100%; height: 100%; padding: 10px; box-sizing: border-box; background: rgba(255, 255, 255, 0.9); text-align: center } /* -2px para garantir que cubra a seta para cima */
    .comment .prompt p { font-weight: 500; }
    .comment .prompt button { user-select: none; }
    .comment .prompt button:not(:last-child) { margin-right: 10px }
    .comment .comment-error p:before { padding-right: 5px; content: "❗" }
    @media(max-width: 500px) { .comment .prompt { padding: 5px } }
    @media(max-width: 500px) { .comment .prompt p { margin: 0 0 .4em 0 } }
    .comment > .inner > .requires-verification-approval, .comment > .inner > .awaiting-approval-notice { margin: 15px 0; }
    .comment .top-right { position: absolute; top: 0; right: 0; z-index: 2; }
    .comment .top-right:hover { z-index: 9001; }
    .comment .jump-link { padding-right: 5px; vertical-align: baseline; font-size: 12px; text-decoration: none; color: #4f4f4f }
    .comment .jump-link .abs-date { margin-left: 5px; }
    @media(max-width: 500px) { .comment > .inner > .top-right { top: -10px; } } /* se 'top' estiver muito baixo, ficará acima do destaque para ações de administrador */
    .comment .top-right > * { display: inline-block; vertical-align: middle }
    .comment > .inner > .replied { display: inline-block; margin: -7px 5px 0 0; vertical-align: top; }
    .hide-avatars .comment > .inner > .replied { margin: 0 5px 0 0; }
    .children > .comment:not(:first-child) > .replied { opacity: 0.5; }
    .comment .menu { position: relative; padding: 10px 10px 10px 0; user-select: none }
    .comment .menu .menu-btn { cursor: pointer }
    .comment .menu .menu-btn i { display: inline-block; width: 4px; height: 4px; margin: 2px; background: #333; border-radius: 10px }
    .comment .menu.empty .menu-btn { cursor: default; opacity: 0.5; }
    .menu-content { position: absolute; width: 130px; min-width: max-content; padding: 20px; background-color: #fff; box-shadow: 2px 3px 6px rgba(0, 0, 0, 0.10); border-radius: 10px 0 10px 10px; z-index: 9001; }
    .menu-content div { padding: 3px; font-weight: 700; cursor: pointer; font-size: 13px }
    .menu-content div > * { vertical-align: middle; pointer-events: none; }
    .menu-content div > i { margin: 0 5px 0 0; }
    .menu-content div > span { display: inline-block; width: calc(100% - 29px); padding: 7px 10px; box-sizing: border-box; }
    .menu-content div:not(:last-child) > span { border-bottom: 1px solid #dcdcdc; }
    .menu-content.corner-bottom-right { border-radius: 10px 10px 0 10px; }
    @media(max-width: 500px) { .comment .menu { padding: 10px 5px 10px 0; } } /* se 'top' estiver muito baixo, ficará acima do destaque para ações de administrador */
    .comment > .children { margin: 15px 0 0 15px }
    .footer { height: 65px; margin-top: 25px; padding-top: 20px; text-align: center; font-size: 12px; } 
    .footer:not(.empty) { border-top: 1px solid #ccc } 
    .footer a, .footer .logo { vertical-align: top; text-decoration: none; color: #201600; font-weight: bold; font-size: 14px } 
    .footer .logo { margin-top: -2px; padding-right: 2px; } 
    .comment.readonly .comment-vote-options { display: none }
    .search-list { position: absolute; z-index: 4; width: 100%; margin-top: -9px; box-sizing: border-box; border-radius: 0 0 11px 11px; background: #fff; border: 1px solid #bfbfbf; }
    .search-list .cross { position: absolute; top: -11px; right: 0; width: 20px; height: 20px; background-color: #fff; border: 1px solid #bfbfbf; border-right: 0; border-radius: 16px 0 0 16px; cursor: pointer; }
    .search-list .search-entry { padding: 5px 10px; cursor: pointer; }
    .search-list .search-entry.last { border-radius: 0 0 11px 11px }
    .search-list .search-entry img { width: 20px; height: 20px; margin-right: 3px; vertical-align: middle; border-radius: 20px; }
    .search-list .search-entry > * { pointer-events: none; }
    .search-list .search-entry:hover, .search-list .search-entry.kb-select, .search-list .cross:hover { background-color: #eee; }
    .comment-input.search-list-open .reply-button-wrapper { display: none; }
    .avatar-wrapper .activity-icon { position: absolute; top: 4px; right: 4px; }
    .activity-icon { width: 8px; height: 8px; border-radius: 10px; }
    .username .activity-icon { position: relative; top: 2px; display: none; margin-right: 5px; }
    .activity-icon.online {  display: inline-block; background: lime; box-shadow: inset 0 2px 2px rgba(0, 0, 0, 0.2);  }
    .hide-avatars .activity-icon.online { top: -1px;  }
    @media(max-width: 500px) { .activity-icon { width: 5px; height: 5px;  }  }
[inline-code-end]