// Grit City Builders — interactions
(function () {
  "use strict";

  // Sticky header state
  var header = document.querySelector(".site-header");
  var onScroll = function () {
    if (!header) return;
    header.classList.toggle("scrolled", window.scrollY > 40);
  };
  window.addEventListener("scroll", onScroll, { passive: true });
  onScroll();

  // Mobile nav
  var toggle = document.querySelector(".nav-toggle");
  if (toggle) {
    toggle.addEventListener("click", function () {
      document.body.classList.toggle("nav-open");
      var open = document.body.classList.contains("nav-open");
      toggle.setAttribute("aria-expanded", open ? "true" : "false");
    });
    document.querySelectorAll(".nav a").forEach(function (a) {
      a.addEventListener("click", function () {
        document.body.classList.remove("nav-open");
        toggle.setAttribute("aria-expanded", "false");
      });
    });
  }

  // Reveal on scroll
  var reveals = document.querySelectorAll(".reveal");
  if ("IntersectionObserver" in window && reveals.length) {
    var io = new IntersectionObserver(function (entries) {
      entries.forEach(function (e) {
        if (e.isIntersecting) { e.target.classList.add("in"); io.unobserve(e.target); }
      });
    }, { threshold: 0.12 });
    reveals.forEach(function (el) { io.observe(el); });
  } else {
    reveals.forEach(function (el) { el.classList.add("in"); });
  }

  // Lightbox gallery
  var zoomables = Array.prototype.slice.call(document.querySelectorAll("[data-zoom]"));
  if (zoomables.length) {
    var box = document.createElement("div");
    box.className = "lightbox";
    box.setAttribute("role", "dialog");
    box.setAttribute("aria-modal", "true");
    box.innerHTML =
      '<button class="lightbox__close" aria-label="Close">&times;</button>' +
      '<button class="lightbox__nav prev" aria-label="Previous">&#8249;</button>' +
      '<button class="lightbox__nav next" aria-label="Next">&#8250;</button>' +
      '<img alt="">' +
      '<div class="lightbox__cap"></div>';
    document.body.appendChild(box);
    var lbImg = box.querySelector("img");
    var lbCap = box.querySelector(".lightbox__cap");
    var current = 0;

    var show = function (i) {
      current = (i + zoomables.length) % zoomables.length;
      var el = zoomables[current];
      var full = el.getAttribute("data-zoom") || el.getAttribute("src");
      lbImg.src = full;
      lbImg.alt = el.getAttribute("alt") || "";
      lbCap.textContent = el.getAttribute("data-caption") || el.getAttribute("alt") || "";
    };
    var open = function (i) { show(i); box.classList.add("open"); document.body.style.overflow = "hidden"; };
    var close = function () { box.classList.remove("open"); document.body.style.overflow = ""; };

    zoomables.forEach(function (el, i) {
      el.addEventListener("click", function () { open(i); });
    });
    box.querySelector(".lightbox__close").addEventListener("click", close);
    box.querySelector(".prev").addEventListener("click", function () { show(current - 1); });
    box.querySelector(".next").addEventListener("click", function () { show(current + 1); });
    box.addEventListener("click", function (e) { if (e.target === box) close(); });
    document.addEventListener("keydown", function (e) {
      if (!box.classList.contains("open")) return;
      if (e.key === "Escape") close();
      if (e.key === "ArrowLeft") show(current - 1);
      if (e.key === "ArrowRight") show(current + 1);
    });
  }

  // Contact form (progressive enhancement over Formspree)
  var form = document.getElementById("contact-form");
  if (form) {
    var status = form.querySelector(".form-status");
    form.addEventListener("submit", function (e) {
      var action = form.getAttribute("action") || "";
      // If no live endpoint configured, fall back to a mailto draft.
      if (action.indexOf("FORM_ID") !== -1 || action === "") {
        e.preventDefault();
        var get = function (n) { var f = form.elements[n]; return f ? encodeURIComponent(f.value) : ""; };
        var body =
          "Name: " + get("name") + "%0D%0A" +
          "Email: " + get("email") + "%0D%0A" +
          "Phone: " + get("phone") + "%0D%0A" +
          "Address: " + get("address") + "%0D%0A%0D%0A" +
          "Project details:%0D%0A" + get("message");
        var subject = encodeURIComponent(form.elements["subject"] ? form.elements["subject"].value : "Project inquiry");
        window.location.href = "mailto:gritcitybuilders@gmail.com?subject=" + subject + "&body=" + body;
        return;
      }
      // AJAX submit to Formspree
      e.preventDefault();
      var data = new FormData(form);
      fetch(action, { method: "POST", body: data, headers: { Accept: "application/json" } })
        .then(function (r) {
          if (r.ok) {
            form.reset();
            if (status) { status.textContent = "Thanks — your message is on its way. We'll be in touch soon."; status.className = "form-status show ok"; }
          } else {
            throw new Error("bad response");
          }
        })
        .catch(function () {
          if (status) { status.textContent = "Something went wrong. Please email gritcitybuilders@gmail.com directly."; status.className = "form-status show err"; }
        });
    });
  }

  // Footer year
  var y = document.getElementById("year");
  if (y) y.textContent = new Date().getFullYear();
})();
