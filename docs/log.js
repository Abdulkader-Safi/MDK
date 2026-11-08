// Fills the table on releases.html and devlog.html from releases.md or devlog.md.
// Each "## " heading is one row: "tag · date · title" for releases,
// "date · title" or "date · tags · title" for the devlog. The text under it is markdown.
(function () {
    var table = document.querySelector("[data-log]");
    if (!table) return;
    var kind = table.dataset.log;
    var releases = "https://github.com/Abdulkader-Safi/MDK/releases";

    function el(tag, className, text) {
        var node = document.createElement(tag);
        if (className) node.className = className;
        if (text) node.textContent = text;
        return node;
    }

    function tagLink(tag, latest) {
        if (tag === "next") return el("span", "ver next", "next");
        var a = el("a", latest ? "ver latest" : "ver", tag);
        a.href = releases + "/tag/" + tag;
        return a;
    }

    function cell(row, child) {
        var td = el("td");
        if (child) td.appendChild(child);
        row.appendChild(td);
        return td;
    }

    function notes(title, markdown) {
        var div = el("div");
        div.appendChild(el("b", "", title));
        var body = el("div", "md");
        // The markdown is this repo's own releases.md and devlog.md, not visitor input.
        body.innerHTML = window.marked ? marked.parse(markdown) : "";
        if (!window.marked) body.textContent = markdown;
        div.appendChild(body);
        return div;
    }

    function downloads(tag) {
        var v = tag.replace(/^v/, "");
        var files = el("div", "files");
        [
            ["macOS", "mdk_" + v + "_aarch64.dmg"],
            ["Windows", "mdk_" + v + "_x64-setup.exe"],
            ["Linux", "mdk_" + v + "_x86_64.AppImage"],
        ].forEach(function (f) {
            var a = el("a", "", f[0]);
            a.href = releases + "/download/" + tag + "/" + f[1];
            files.appendChild(a);
        });
        return files;
    }

    fetch(kind + ".md", { cache: "no-cache" })
        .then(function (r) {
            if (!r.ok) throw new Error(r.status);
            return r.text();
        })
        .then(function (md) {
            var body = table.querySelector("tbody");
            md.split(/^## /m)
                .slice(1)
                .forEach(function (section, i) {
                    var nl = section.indexOf("\n");
                    var head = (nl < 0 ? section : section.slice(0, nl)).split(" · ");
                    var text = nl < 0 ? "" : section.slice(nl + 1).trim();
                    var row = el("tr");
                    row.style.setProperty("--i", i);
                    if (kind === "releases") {
                        var tag = head[0].trim();
                        row.dataset.tag = tag;
                        cell(row, tagLink(tag, i === 0));
                        cell(row, null).appendChild(el("span", "when", head[1] || ""));
                        cell(row, notes(head.slice(2).join(" · "), text));
                        cell(row, downloads(tag));
                    } else {
                        var tags = head.length > 2 ? head[1].trim().split(/\s+/) : [];
                        var title = head.slice(head.length > 2 ? 2 : 1).join(" · ");
                        cell(row, null).textContent = head[0];
                        var td = cell(row, null);
                        tags.forEach(function (t) {
                            td.appendChild(tagLink(t, false));
                        });
                        cell(row, notes(title, text));
                    }
                    body.appendChild(row);
                });
        })
        .catch(function () {
            var p = el("p", "log-more", "Could not load the list. Read it on GitHub: ");
            var a = el("a", "", kind + ".md");
            a.href = "https://github.com/Abdulkader-Safi/MDK/blob/main/docs/" + kind + ".md";
            p.appendChild(a);
            table.after(p);
        });
})();
