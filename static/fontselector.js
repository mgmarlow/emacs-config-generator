(function (document) {
  var width;
  var body = document.body;

  var container = document.createElement("span");
  container.innerHTML = Array(100).join("wi");
  container.style.cssText = [
    "position:absolute",
    "width:auto",
    "font-size:128px",
    "left:-99999px",
  ].join(" !important;");

  var getWidth = function (fontFamily) {
    container.style.fontFamily = fontFamily;

    body.appendChild(container);
    width = container.clientWidth;
    body.removeChild(container);

    return width;
  };

  // Pre compute the widths of monospace, serif & sans-serif
  // to improve performance.
  var monoWidth = getWidth("monospace");
  var serifWidth = getWidth("serif");
  var sansWidth = getWidth("sans-serif");

  window.isFontAvailable = function (font) {
    return (
      monoWidth !== getWidth(font + ",monospace") ||
      sansWidth !== getWidth(font + ",sans-serif") ||
      serifWidth !== getWidth(font + ",serif")
    );
  };
})(document);

const allFonts = [
  "SF Mono",
  "Menlo",
  "Consolas",
  "Segoe UI Mono",
  "Ubuntu Mono",
  "Roboto Mono",
  "Monaco",
  "Courier New",
  "Courier",
  "foo",
];

const inputEl = document.getElementById("font_family");
const availableFonts = allFonts.filter(window.isFontAvailable);
if (inputEl && availableFonts.length > 0) {
  inputEl.value = availableFonts[0];
}
