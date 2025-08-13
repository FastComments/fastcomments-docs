Now we're going to copy our FastComments code. Try using the "Copy" button on the right, as the code is quite large to work properly
with GoHighLevel:

[inline-code-attrs-start title = 'GoHighLevel FastComments Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script id="fastcomments-embed" src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
  (function () {
      const tenantId = "demo";
      const VALID_PATTERNS = ['/post'];
      const SCRIPT_ID = 'fastcomments-embed';
      const SCRIPT_SRC = 'https://cdn.fastcomments.com/js/embed-v2.min.js';

      // Function to ensure script is loaded
      function ensureScriptLoaded() {
          return new Promise((resolve) => {
              // Check if script tag already exists
              let scriptTag = document.getElementById(SCRIPT_ID);

              if (!scriptTag) {
                  console.log('FastComments: Script tag not found, adding dynamically...');
                  scriptTag = document.createElement('script');
                  scriptTag.id = SCRIPT_ID;
                  scriptTag.src = SCRIPT_SRC;
                  scriptTag.async = true;

                  scriptTag.onload = () => {
                      console.log('FastComments: Script loaded successfully');
                      resolve();
                  };

                  scriptTag.onerror = () => {
                      console.error('FastComments: Failed to load script');
                      resolve(); // Resolve anyway to prevent hanging
                  };

                  document.head.appendChild(scriptTag);
              } else if (window.FastCommentsUI) {
                  // Script tag exists and FastCommentsUI is already loaded
                  console.log('FastComments: Script already loaded');
                  resolve();
              } else {
                  // Script tag exists but FastCommentsUI not ready yet
                  console.log('FastComments: Waiting for script to initialize...');
                  scriptTag.addEventListener('load', () => {
                      resolve();
                  });

                  // Fallback in case the script is already loading
                  const checkInterval = setInterval(() => {
                      if (window.FastCommentsUI) {
                          clearInterval(checkInterval);
                          resolve();
                      }
                  }, 100);

                  // Timeout after 10 seconds
                  setTimeout(() => {
                      clearInterval(checkInterval);
                      console.warn('FastComments: Script load timeout');
                      resolve();
                  }, 10000);
              }
          });
      }

      // History API modifications for SPA support
      const oldPushState = history.pushState;
      history.pushState = function pushState() {
          const ret = oldPushState.apply(this, arguments);
          window.dispatchEvent(new Event('pushstate'));
          window.dispatchEvent(new Event('locationchange'));
          return ret;
      };

      const oldReplaceState = history.replaceState;
      history.replaceState = function replaceState() {
          const ret = oldReplaceState.apply(this, arguments);
          window.dispatchEvent(new Event('replacestate'));
          window.dispatchEvent(new Event('locationchange'));
          return ret;
      };

      window.addEventListener('popstate', () => {
          window.dispatchEvent(new Event('locationchange'));
      });

      let lastInstance;

      // Main render function
      async function render() {
          let rendered = false;

          // Ensure script is loaded before proceeding
          await ensureScriptLoaded();

          function tryNext() {
              if (rendered) {
                  return;
              }

              // Check if we should render on this page
              if (!VALID_PATTERNS.some(function(pattern) {
                  return window.location.pathname.includes(pattern);
              })) {
                  console.log('FastComments: Not set to load on this page. Waiting.');
                  setTimeout(tryNext, 1000);
                  return;
              }

              // Double-check FastCommentsUI is available
              if (!window.FastCommentsUI) {
                  console.log('FastComments: FastCommentsUI not ready, waiting...');
                  setTimeout(tryNext, 300);
                  return;
              }

              // Try to find container with multiple selectors
              let container = document.querySelector('#post-body');
              if (!container) {
                  container = document.querySelector('#content-container #content-container #post-description');
              }
              if (!container) {
                  container = document.querySelector('#post-description');
              }
              if (!container) {
                  container = document.querySelector('#content-container');
              }
              if (!container) {
                  container = document.querySelector('.post-description'); // mobile
              }

              if (container) {
                  console.log('FastComments: Container found, updating...');

                  // Remove existing wrapper if present
                  const existingWrapper = document.querySelector('.fastcomments-wrapper');
                  if (existingWrapper) {
                      existingWrapper.remove();
                  }

                  if (lastInstance) {
                      lastInstance.destroy();
                  }

                  // Create new wrapper
                  const target = document.createElement('div');
                  target.classList.add('fastcomments-wrapper', 'col-span-8');
                  container.append(target);

                  // Initialize FastComments
                  lastInstance = FastCommentsUI(target, {
                      tenantId,
                      showLiveRightAway: true
                  });

                  rendered = true;

                  // Monitor if container gets removed
                  const interval = setInterval(function() {
                      const doesContainerStillExist = document.querySelector('.fastcomments-wrapper');
                      if (!doesContainerStillExist) {
                          rendered = false;
                          tryNext();
                          clearInterval(interval);
                      } else {
                          console.log('Container still exists...');
                      }
                  }, 1000);
              } else {
                  console.log('FastComments: Container not found, waiting...');
                  setTimeout(tryNext, 300);
              }
          }

          tryNext();
      }

      // Initial render
      render();

      // Re-render on location change
      window.addEventListener('locationchange', function () {
          console.log('Updating FastComments.');
          render();
      });
  })();
</script>
[inline-code-end]

You can configure the `VALID_PATTERNS` variable to set which URL routes the comments should show. By default, it will show
on pages that contain `/post` in the URL.
