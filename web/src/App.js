import React, { Component, useState, useCallback, useEffect } from "react";
import { MainContext, useValue, useMainContext } from './hooks/context'

const exprs = [
  ["simple pipe", "pipes will turn into parentheses", "42 | f"],
  ["input as first arg", "for functions with multiple args, the input will be passed as the first one", "42 | f | g(3) | h"],
  ["input as last arg", "if you want to determine the arg position, use the x -> syntax", "42 | f | x -> g(3, x) | h"],
  ["newlines", "you can use indented newlines to improve readability", `42 
  | f 
  | x -> g(3, x) 
  | h`],
  ["named variables", "you can split a expression into multiple named ones", `x = 42 | f

y = 53 | g(3)

x + y | sum`],
  ["escaping", "if you need to use a | inside your expressions, escape it with \\", `53 | x -> g(x \\|\\| true)`],
]

function Example({ id }) {
  const { convert } = useMainContext()
  const [value, setValue] = useState(exprs[id][2])

  const onChange = useCallback((event) => {
    const v = event.target.value;
    console.log({ v, c: convert(v) })
    setValue(v)
  }, [setValue, convert])

  return (
    <section>
      <div className="section-container">
        <div className="meta-area">
          <h2 className="section-title">{id + ". " + exprs[id][0]}
            <code className="">{exprs[id][1]}</code>
          </h2>
        </div>

        <div className="ex-section ex1">
          <textarea className="code-container" onChange={onChange} value={value} />
          <div className="code-area">
            <div className="code-container html">{convert(value)}</div>
          </div>
        </div>
      </div>
    </section>
  )
}

function Examples() {
  return (
    <div>
      {
        exprs.map((value, index) => (<Example id={index} />))
      }
    </div>
  )
}

function Header() {
  return (
    <div class="meta">
      <h1>Pipers</h1>
      <p>
        Use pipe expressions in your PromQL queries and more!
      </p>
    </div>
  )
}
export default function App() {
  const value = useValue()

  return (
    <MainContext.Provider value={value}>
      <div>
        <Header />
        <Examples />
      </div>
    </MainContext.Provider >
  )
}
