#import "@preview/simple-technical-resume:0.1.1": *

#let name = "{{ name }}"
#let phone = "{{ phone }}"
#let email = "{{ email }}"

#let github = "{{ github }}"
#let personal-site = "{{ site }}"
#let linkedin = "{{ linkedin }}"

#show link: set text(fill: blue)
#show link: underline

#show: resume.with(
  top-margin: 0.3in,
  personal-info-font-size: 9.6pt,
  author-position: center,
  personal-info-position: center,
  author-name: name,
  phone: phone,
  email: email,
  website: personal-site,
  // linkedin-user-id: linkedin,
  github-username: github,
)

{#
  FIXME: tera.render fails when the variables being iterated through via for
  loops don't exist; wrap the entire expression in an 'if \_.is_some()'
#}

#custom-title("Education")[
{%- for edu in institutes %}
  #education-heading(
    "{{ edu.institute }}",
    "{{ edu.location }}",
    "{{ edu.degree }}",
    "{{ edu.major }}",
    datetime(year: {{ edu.start_date.year }}, month: {{ edu.start_date.month }}, day: {{ edu.start_date.day }}),
    datetime(year: {{ edu.end_date.year }}, month: {{ edu.end_date.month }}, day: {{ edu.end_date.day }}),
  )[
{%- for line in edu.desc %}
  - {{ line }}
{%- endfor %}
  ]
{% endfor -%}
]

#custom-title("Experience")[
{%- for wrk in workplaces %}
  #work-heading(
    "{{ wrk.title }}",
    "{{ wrk.company }}",
    "",
    datetime(year: {{ wrk.start_date.year }}, month: {{ wrk.start_date.month }}, day: {{ wrk.start_date.day }}),
    datetime(year: {{ wrk.end_date.year }}, month: {{ wrk.end_date.month }}, day: {{ wrk.end_date.day }}),
  )[
{%- for line in wrk.desc %}
  - {{ line }}
{%- endfor %}
  ]
{% endfor -%}
]

#custom-title("Projects")[
{%- for proj in projects %}
  #project-heading(
    "{{ proj.title }}",
    stack: "{{ proj.stack }}",
    project-url: "{{ proj.url }}"
  )[
{%- for line in proj.desc %}
  - {{ line }}
{%- endfor %}
  ]
{% endfor -%}
]

#custom-title("Smaller Projects/Tools")[
{%- for proj in small_projects %}
  #project-heading(
    "{{ proj.title }}",
    project-url: "{{ proj.url }}"
  )[
{%- for line in proj.desc %}
  - {{ line }}
{%- endfor %}
  ]
{% endfor -%}
]

#custom-title("Hobby Projects")[
{%- for proj in hobby_projects %}
  #project-heading(
    "{{ proj.title }}",
    project-url: "{{ proj.url }}"
  )[
{%- for line in proj.desc %}
  - {{ line }}
{%- endfor %}
  ]
{% endfor -%}
]

#custom-title("Skills")[
  #skills()[
    - *Languages:* {{ skills.langs }}
    - *Operating Systems:* {{ skills.operating_systems }}
    - *Developer Tools:* {{ skills.dev_tools }}
    - *Libraries:* {{ skills.libraries }}
  ]
]

