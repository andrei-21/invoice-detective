use invoice_detective::{InvoiceDetective, RecipientNode, ServiceKind};
use rocket::{get, launch, routes};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context![])
}

#[get("/<invoice>")]
fn invoice(invoice: &str) -> Template {
    let invoice_detective = InvoiceDetective::new().unwrap();
    let findings = invoice_detective.investigate(invoice).unwrap();

    let recipient = findings.recipient;
    let payee = findings.payee;
    let route_hints = findings.route_hints;

    let (custody, service, name, id) = match recipient {
        RecipientNode::Custodial { custodian } => (
            "Custodial",
            format_service_kind(&custodian.service),
            custodian.name,
            String::new(),
        ),
        RecipientNode::NonCustodial { id, lsp } => (
            "Non-custodial",
            format_service_kind(&lsp.service),
            lsp.name,
            id,
        ),
        RecipientNode::NonCustodialWrapped { lsp } => (
            "Non-custodial",
            format_service_kind(&lsp.service),
            lsp.name,
            "Wrapped invoice".to_string(),
        ),
        RecipientNode::Unknown => ("Unknown", "", String::new(), String::new()),
    };

    let mempool_space_base_url = "https://mempool.space/lightning/node";
    Template::render(
        "invoice",
        context! { invoice, mempool_space_base_url, route_hints, payee, custody, service, name, id },
    )
}

#[launch]
fn rocket() -> _ {
    // TODO: Customize template directory.
    rocket::build()
        .mount("/", routes![index, invoice])
        .attach(Template::fairing())
}

fn format_service_kind(service: &ServiceKind) -> &'static str {
    match service {
        ServiceKind::BusinessWallet => "Payment processor",
        ServiceKind::ConsumerWallet => "Consumer wallet",
        ServiceKind::Exchange => "Exchange",
        ServiceKind::Lsp => "LSP",
    }
}