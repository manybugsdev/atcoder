(async () => {
  const dir = "test";
  let command = `mkdir -p ${dir}; `;
  const task = location.pathname.split("/").pop();
  for (const node of document.querySelectorAll(".part")) {
    const h = node.querySelector("h3");
    const ht = h.textContent.slice(0, -4);
    const io = ht.slice(0, 3);
    const isInput = io === "入力例";
    const isOutput = io === "出力例";
    if (!isInput && !isOutput) {
      continue;
    }
    const n = ht.slice(4);
    const pre = node.querySelector("pre");
    command += `echo "${pre.textContent.trim()}" | tee -i ${dir}/${task}_${
      isInput ? "in" : "out"
    }_${n}; `;
  }
  await navigator.clipboard.writeText(command);
})();
