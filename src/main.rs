use chrono::Local;
use mathlab::math;
use yew::prelude::*;

pub fn get_year() -> String {
    let input = Local::now().to_string();
    let substring = input.split('-').next().unwrap_or("");
    substring.to_string()
}

#[function_component]
fn App() -> Html {
    let range_asc = math::range(0.0, 15.0, 25, "asc");
    let range_desc = math::range(360.0, 15.0, 25, "desc");

    html! {
        <div>
            <div class="m-8 text-center header">
                <h1 class="p-5 mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white">{"📏 MathLab 📦"}</h1>
                    <p class="mb-6 text-lg font-normal text-gray-500 lg:text-xl sm:px-16 xl:px-48 dark:text-gray-400">{"A Powerful Math Library for Rust"}</p>
                    <a href="https://crates.io/crates/mathlab" target="_blank" class="inline-flex items-center justify-center px-5 py-3 text-base font-medium text-center text-white bg-green-700 rounded-lg hover:bg-green-800 focus:ring-4 focus:ring-green-300 dark:focus:ring-green-900">
                        {"Learn more"}
                        <svg class="w-3.5 h-3.5 ms-2 rtl:rotate-180" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 5h12m0 0L9 1m4 4L9 9"/>
                        </svg>
                    </a>

                    <br />
                    <br />

                    <a href="https://github.com/dr-montasir/mathlab-yew" target="_blank" class="inline-flex items-center justify-center px-5 py-3 text-base font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 dark:focus:ring-blue-900">
                        {"Checkout the demo project"}
                        <svg class="w-4 h-4 ms-2" height="32" aria-hidden="true" viewBox="0 0 24 24" version="1.1" width="32" data-view-component="true">
                            <path fill="#fff" d="M12.5.75C6.146.75 1 5.896 1 12.25c0 5.089 3.292 9.387 7.863 10.91.575.101.79-.244.79-.546 0-.273-.014-1.178-.014-2.142-2.889.532-3.636-.704-3.866-1.35-.13-.331-.69-1.352-1.18-1.625-.402-.216-.977-.748-.014-.762.906-.014 1.553.834 1.769 1.179 1.035 1.74 2.688 1.25 3.349.948.1-.747.402-1.25.733-1.538-2.559-.287-5.232-1.279-5.232-5.678 0-1.25.445-2.285 1.178-3.09-.115-.288-.517-1.467.115-3.048 0 0 .963-.302 3.163 1.179.92-.259 1.897-.388 2.875-.388.977 0 1.955.13 2.875.388 2.2-1.495 3.162-1.179 3.162-1.179.633 1.581.23 2.76.115 3.048.733.805 1.179 1.825 1.179 3.09 0 4.413-2.688 5.39-5.247 5.678.417.36.776 1.05.776 2.128 0 1.538-.014 2.774-.014 3.162 0 .302.216.662.79.547C20.709 21.637 24 17.324 24 12.25 24 5.896 18.854.75 12.5.75Z"></path>
                        </svg>
                    </a>
            </div>

            <div class="flex items-center justify-center min-h-screen">
                <div class="overflow-x-auto">
                    <h2 class="font-extrabold text-center">{"Table 1"}</h2>
                    <table class="min-w-full bg-white shadow-md rounded-xl">
                    <thead>
                        <tr class="text-gray-700 bg-blue-gray-100">
                            <th class="px-4 py-3 text-left">{ "Index" }</th>
                            <th class="px-4 py-3 text-left">{ "x" }<sup>{"o"}</sup></th>
                            <th class="px-4 py-3 text-left">{"sin( x"}<sup>{"o"}</sup>{" )"}</th>
                            <th class="px-4 py-3 text-left">{"cos( x"}<sup>{"o"}</sup>{" )"}</th>
                            <th class="px-4 py-3 text-left">{"tan( x"}<sup>{"o"}</sup>{" )"}</th>
                            <th class="px-4 py-3 text-left">{"csc( x"}<sup>{"o"}</sup>{" )"}</th>
                            <th class="px-4 py-3 text-left">{"sec( x"}<sup>{"o"}</sup>{" )"}</th>
                            <th class="px-4 py-3 text-left">{"cot( x"}<sup>{"o"}</sup>{" )"}</th>
                        </tr>
                    </thead>
                    <tbody class="text-blue-gray-900">
                        { for range_asc.iter().enumerate().map(|(i, &value)| html! {
                        <tr class="border-b border-blue-gray-200">
                            <td class="px-4 py-3">{ i + 1 }</td>
                            <td class="px-4 py-3">{ value }</td>
                            <td class="px-4 py-3">{ math::sin_deg(value) }</td>
                            <td class="px-4 py-3">{ math::cos_deg(value) }</td>
                            <td class="px-4 py-3">{ math::tan_deg(value) }</td>
                            <td class="px-4 py-3">{ math::csc_deg(value) }</td>
                            <td class="px-4 py-3">{ math::sec_deg(value) }</td>
                            <td class="px-4 py-3">{ math::cot_deg(value) }</td>
                        </tr>
                    })}
                    </tbody>
                    </table>
                    <div class="w-full px-4 pt-5 mx-auto mb-8 "></div>
                </div>
            </div>

            <div class="flex items-center justify-center min-h-screen">
                <div class="overflow-x-auto">
                    <h2 class="font-extrabold text-center">{"Table 2"}</h2>
                    <table class="min-w-full bg-white shadow-md rounded-xl">
                    <thead>
                        <tr class="text-gray-700 bg-blue-gray-100">
                            <th class="px-4 py-3 text-left">{ "x" }<sup>{"o"}</sup></th>
                            <th class="px-4 py-3 text-left">{"sin( x"}<sup>{"o"}</sup>{" )"}</th>
                            <th class="px-4 py-3 text-left">{"cos( x"}<sup>{"o"}</sup>{" )"}</th>
                            <th class="px-4 py-3 text-left">{"tan( x"}<sup>{"o"}</sup>{" )"}</th>
                            <th class="px-4 py-3 text-left">{"csc( x"}<sup>{"o"}</sup>{" )"}</th>
                            <th class="px-4 py-3 text-left">{"sec( x"}<sup>{"o"}</sup>{" )"}</th>
                            <th class="px-4 py-3 text-left">{"cot( x"}<sup>{"o"}</sup>{" )"}</th>
                        </tr>
                    </thead>
                    <tbody class="text-blue-gray-900">
                        { for range_desc.iter().map(|&value| html! {
                        <tr class="border-b border-blue-gray-200">
                            <td class="px-4 py-3">{ value }</td>
                            <td class="px-4 py-3">{ math::sin_deg(value) }</td>
                            <td class="px-4 py-3">{ math::cos_deg(value) }</td>
                            <td class="px-4 py-3">{ math::tan_deg(value) }</td>
                            <td class="px-4 py-3">{ math::csc_deg(value) }</td>
                            <td class="px-4 py-3">{ math::sec_deg(value) }</td>
                            <td class="px-4 py-3">{ math::cot_deg(value) }</td>
                        </tr>
                    })}
                    </tbody>
                    </table>
                    <div class="w-full px-4 pt-5 mx-auto mb-8 ">
                        <div class="py-1 text-sm text-center text-gray-700">
                            {"Copyright "}
                            if get_year() == "2024" {
                                { "2024" }
                            } else {
                                {"2024"}{" - "}{ get_year() }
                            }
                            {" Ⓒ "} <a class="font-semibold text-gray-700" href="https://github.com/dr-montasir/mathlab" target="_blank">{"Dr. Montasir"}</a>{". All Rights Reserved."}
                        </div>
                    </div>
                </div>
            </div>

        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
