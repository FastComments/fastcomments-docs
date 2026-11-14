एक्सटेंशन ऑब्जेक्ट निम्नलिखित परिभाषा से मिलकर बना है:

<!-- यदि आप इसे अपडेट करना चाहते हैं, तो comment-ui-core को अपडेट करना न भूलें -->
[inline-code-attrs-start title = 'Extension Object JSDoc'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
/**
 * FastCommentsUI एक्सटेंशन ऑब्जेक्ट। कुछ कंपोनेंट्स को lazy-load करने के लिए उपयोग किया जाता है। उदाहरण के लिए, रिव्यू सिस्टम सभी
 * ग्राहकों द्वारा उपयोग नहीं किया जाता, इसलिए हम उस एक्सटेंशन को तभी लोड करते हैं जब हमें उसकी आवश्यकता हो।
 *
 * @typedef {Object} FastCommentsUIExtension
 * @property {string} id
 * @property {Element} scriptNode
 * @property {Element} root - विजेट का रूट DOM नोड।
 * @property {string} [css]
 * @property {Object} config - FastComments कॉन्फ़िग ऑब्जेक्ट।
 * @property {Object} commentsById - आईडी के अनुसार सभी कमेंट्स वाले ऑब्जेक्ट का संदर्भ, जो अपडेट रखा जाता है।
 * @property {Object} translations - सभी अनुवादों का संदर्भ।
 * @property {Function} reRenderComment - एक फ़ंक्शन का संदर्भ जिसे किसी कमेंट को फिर से रेंडर करने के लिए कॉल किया जा सकता है।
 * @property {Function} removeCommentAndReRender - एक फ़ंक्शन का संदर्भ जिसे किसी कमेंट को मेमोरी से हटाने और DOM के उपयुक्त भाग को फिर से रेंडर करने के लिए कॉल किया जा सकता है।
 * @property {Function} newBroadcastId - एक फ़ंक्शन का संदर्भ जिसे नया broadcast id बनाने और उसे अनदेखा करने के लिए स्थानीय broadcast ids की सूची में जोड़ने के लिए कॉल किया जा सकता है।
 * @property {Function} [withSessionId] - एक रिक्वेस्ट URL और एक कॉलबैक के साथ कॉल करें। जब थर्ड-पार्टी कुकीज़ ब्लॉक हों (वही पाथ जो कमेंट और वोट रिक्वेस्ट उपयोग करते हैं) तो URL के साथ व्यूअर की session id जोड़कर कॉलबैक को इनवोक करता है, अन्यथा URL अपरिवर्तित रहता है। SSO के लिए उपयोग नहीं किया जाता।
 * @property {FastCommentsUIExtensionSetupEventHandlers} [setupEventHandlers]
 * @property {FastCommentsUIExtensionPrepareCommentForSavingCallback} [prepareCommentForSaving] - पोस्ट होने वाले कमेंट के साथ कॉल किया जाता है। सबमिट रद्द करने के लिए false लौटाएं (उदाहरण के लिए जब जुड़ा पोल अधूरा हो)।
 * @property {FastCommentsUIExtensionNewCommentCallback} [newComment]
 * @property {FastCommentsUIExtensionReplyAreaFilter} [replyAreaFilter] - कमेंट एरिया के लिए HTML फ़िल्टर।
 * @property {FastCommentsUIExtensionWidgetFilter} [widgetFilter] - रेंडर पर पूरे विजेट के लिए HTML फ़िल्टर।
 * @property {FastCommentsUIExtensionCommentTopFilter} [commentFilter] - रेंडर से पहले प्रत्येक कमेंट के लिए HTML फ़िल्टर।
 * @property {FastCommentsUIExtensionReplyAreaFilter} [commentMenuFilter] - रेंडर से पहले प्रत्येक कमेंट मेनू के लिए HTML फ़िल्टर।
 * @property {FastCommentsUIExtensionMenuFilter} [menuFilter] - रेंडर पर पूरे विजेट के लिए HTML फ़िल्टर।
 * @property {FastCommentsUIExtensionReplyAreaTop} [replyAreaTop] - (LEGACY) रिप्लाई एरिया के शीर्ष पर जोड़ने के लिए HTML लौटाएं।
 * @property {FastCommentsUIExtensionWidgetTopCallback} [widgetTop] - (LEGACY) विजेट के शीर्ष पर जोड़ने के लिए HTML लौटाएं।
 * @property {FastCommentsUIExtensionCommentTopCallback} [commentTop] - (LEGACY) कमेंट एलिमेंट के शीर्ष पर जोड़ने के लिए HTML लौटाएं।
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentBottom] - (LEGACY) कमेंट एलिमेंट के निचले भाग पर जोड़ने के लिए HTML लौटाएं।
 * @property {FastCommentsUIExtensionCommentBottomCallback} [commentContentBottom] - कमेंट टेक्स्ट के बाद, कमेंट कंटेंट एलिमेंट के अंदर जोड़ने के लिए HTML लौटाएं (पोल्स द्वारा उपयोग किया जाता है)।
 * @property {Function} [replyAreaInputBottom] - कमेंट इनपुट फ्रेम के अंदर, टेक्स्ट इनपुट के नीचे जोड़ने के लिए HTML लौटाएं (इन-प्लेस पोल एडिटर के लिए पोल्स द्वारा उपयोग किया जाता है)। पैरेंट कमेंट आईडी प्राप्त करता है, या रूट रिप्लाई बॉक्स के लिए null।
 * @property {Function} [onPollUpdate] - पेज पर किसी पोल के वोट काउंट बदलने पर लाइव इवेंट के साथ कॉल किया जाता है।
 * @property {Function} isSiteAdmin - लौटाता है कि व्यूअर टेनेंट का एडमिन या मॉडरेटर है या नहीं। पहली फ़ेच के बाद ज्ञात होता है।
 * @property {FastCommentsUIExtensionMenuBottomCallback} [menuBottom] - (LEGACY) प्रत्येक कमेंट के लिए मेनू एलिमेंट के निचले भाग पर जोड़ने के लिए HTML लौटाएं।
 * @property {FastCommentsUIExtensionRenderCallback} [onRender]
 * @property {FastCommentsUIExtensionConnectionStatusCallback} [onLiveConnectionStatusUpdate]
 * @property {FastCommentsUIExtensionInitialRenderCallback} [onInitialRenderComplete]
 * @property {FastCommentsUIExtensionPresenceUpdateCallback} [onPresenceUpdate]
 */
   
/**
 * @callback FastCommentsUIExtensionSetupEventHandlers
 * @param {Element} element - रूट एलिमेंट।
 * @param {Object.<string, Function>} clickListeners - क्लिक के लिए इवेंट हैंडलर, क्लास नाम के अनुसार, जिन्हें संदर्भ द्वारा संशोधित किया जा सकता है।
 * @returns void
 */

/**
 * @callback FastCommentsUIExtensionWidgetTopCallback
 * @param {Object} moduleData
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionWidgetFilter
 * @param {Object} moduleData
 * @param {Object} html
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionCommentTopCallback
 * @param {Object} comment
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionCommentTopFilter
 * @param {Object} comment
 * @param {string} html
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionCommentBottomCallback
 * @param {Object} comment
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionMenuBottomCallback
 * @param {Object} comment
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionMenuFilter
 * @param {Object} comment
 * @param {string} html
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionRenderCallback
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionConnectionStatusCallback
 * @param {boolean} isConnected
 * @returns {void}
 */

/**
 * @callback FastCommentsUIExtensionInitialRenderCallback
 * @returns {void}
 */

/**
 * @callback FastCommentsUIExtensionReplyAreaTop
 * @param {Object|null} currentUser
 * @param {boolean} isSaving
 * @param {boolean} isReplyOpen
 * @param {string|null} parentId
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionReplyAreaFilter
 * @param {Object|null} currentUser
 * @param {boolean} isSaving
 * @param {boolean} isReplyOpen
 * @param {string|null} parentId
 * @param {string|null} html
 * @returns {string}
 */

/**
 * @callback FastCommentsUIExtensionPrepareCommentForSavingCallback
 * @param {Object} comment
 * @param {string} parentId
 */

/**
 * @callback FastCommentsUIExtensionNewCommentCallback
 * @param {Object} comment
 */

/**
 * @callback FastCommentsUIExtensionPresenceUpdateCallback
 * @param {Object} update
 */
[inline-code-end]