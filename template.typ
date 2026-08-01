// adapted from: https://git.dblsaiko.net/web/tree/global/common.typ

#let image(source, width: "auto", height: "auto") = html.elem("img", attrs: (
  src: source,
  width: width,
  height: height,
))


#let page(body) = {
  let page_title = sys.inputs.at("title", default: "Unnamed")
  set document(title: page_title)

  let comments = sys.inputs.at("comments", default: none)

  html.html(lang: "en", {
    html.head({
      html.meta(charset: "utf-8")
      html.meta(
        name: "viewport",
        content: "width=device-width, initial-scale=1",
        charset: "utf-8",
        http-equiv: "x-ua-compatible",
      )
      html.title(page_title)

      // for href in image-res {
      //     html.elem("link", attrs: (rel: "preload", href: href, "as": "image"))
      // }

      html.link(rel: "stylesheet", href: "/style.css")
      html.link(
        rel: "alternate",
        type: "application/rss+xml",
        title: "Erin Staugaard's Blog",
        href: "/blog/feed",
      )

      // for ss in stylesheets {
      //     html.link(rel: "stylesheet", href: ss)
      // }
    })

    html.body({
      show outline.entry: it => {
        html.span(class: "prefix", it.prefix())
        link(it.element.location(), it.body())
      }

      html.elem("nav", attrs: (class: "navbar"))[
        #html.a(href: "/")[Home]
        #html.a(href: "/blog")[Blog]
      ]

      html.elem("div", attrs: (class: "content"))[
        #title()
        #body
      ]

      if comments != none {
        html.elem("div", attrs: (
          class: "atproto-embed",
          data-uri: comments,
          data-mode: "discussion",
          data-width: "100%",
          data-max-width: "728px",
          data-show-likes: "true",
          data-show-reposts: "true",
          data-show-replies: "true",
          data-show-quotes: "false",
          data-show-bookmarks: "true",
          data-show-metrics: "true",
          data-show-timestamp: "true",
          data-show-actions: "true",
          data-show-reply-context: "true",
          data-show-embeds: "true",
          data-show-images: "true",
          data-show-video: "true",
          data-show-external: "true",
          data-show-quote-posts: "false",
          data-external-layout: "vertical",
          data-show-badges: "true",
          data-show-labels: "true",
          data-show-reply-quote-labels: "true",
          data-show-main-post: "false",
          data-show-liked-by: "false",
          data-show-replies-tab: "true",
          data-show-quotes-tab: "false",
          data-show-tabs: "false",
          data-show-sort: "true",
          data-show-join-button: "true",
          data-replies-sort: "oldest",
          data-dark: "true",
        ))

        html.script(
          src: "https://cdn.jsdelivr.net/npm/atproto-embed@latest/dist/embed.js",
        )
      }
    })
  })
}
