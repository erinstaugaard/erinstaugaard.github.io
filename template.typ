// adapted from: https://git.dblsaiko.net/web/tree/global/common.typ

#let image(source, width: "auto", height: "auto") = html.elem("img", attrs: (
  src: source,
  width: width,
  height: height,
))


#let page(body) = {
  let page_title = sys.inputs.at("title", default: "Unnamed")
  set document(title: page_title)

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
    })
  })
}
