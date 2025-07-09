use std::collections::HashMap;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::*;

/// Параметры создания платежа (`POST /v3/payments`).
///
/// Большинство полей опциональны — передаёшь только то,
/// что нужно под твой сценарий (одностадийный/двустадийный,
/// сохранение карты, чек, сплит-платёж и т. д.).
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RequestCreatePayments {
    /// Сумма к списанию с плательщика
    /// (объект `Amount` — `value` + `currency`, обязательный).
    pub amount: Amount,

    /// Описание транзакции (до 128 символов) — видно в кабинете
    /// магазина и плательщику в выписке.
    pub description: Option<String>,

    /// Данные для формирования фискального чека 54-ФЗ.
    pub receipt: Option<Receipt>,

    /// Если нужно провести платёж в пользу другого суб-магазина
    /// (маркетплейс-схема) — указываешь `gateway_id`.
    pub recipient: Option<Recipient>,

    /// Токен, полученный в мобильном/JS-SDK
    /// (`payment_token` ↔ старый `payment_token`/`cps_token`).
    pub payment_token: Option<String>,

    /// Данные о способе оплаты, если он ещё **не** сохранён
    /// (пример: `{ "type": "bank_card" }`).
    pub payment_method_data: Option<PaymentMethodData>,

    /// Сценарий подтверждения (`redirect`, `embedded`,
    /// `mobile_application`, `qr_code`, …).
    pub confirmation: Option<Confirmation>,

    /// `true` — сохранить карту/кошелёк покупателя
    /// для повторных списаний. Требует договора «Сохранённые платежи».
    pub save_payment_method: Option<bool>,

    /// `true` — списать деньги сразу (одностадийно);
    /// `false` — только заблокировать, позже вызвать `POST /capture`
    /// (двустадийная схема). :contentReference[oaicite:0]{index=0}
    pub capture: Option<bool>,

    /// IP-адрес клиента в формате IPv4/IPv6.
    pub client_ip: Option<String>,

    /// Произвольные метаданные (до 16 KB), хранятся «как есть».
    pub metadata: Option<HashMap<String, String>>,

    /// Данные авиабилета (если продаёшь перелёты).
    pub airline: Option<Airline>,

    /// Список распределений денег между суб-аккаунтами
    /// (сплит-платёж).
    pub transfers: Option<Vec<Transfer>>,

    /// Объект Safe Deal (резервирование средств до выполнения услуги).
    pub deal: Option<Deal>,

    /// Идентификатор клиента в твоей CRM/системе лояльности.
    pub merchant_customer_id: Option<String>,

    /// Доп-настройки заказа для Системы быстрых платежей (СБП)
    /// или BNPL-схем. Если не используешь — опусти.
    pub payment_order: Option<Value>,

    /// Объект получателя при передаче реквизитов напрямую
    /// (редкий кейс — escrow, C2C-маркетплейсы).
    pub receiver: Option<Value>,
}

/// Полный набор данных для чека 54-ФЗ.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Receipt {
    /// Данные покупателя (email, телефон, ИНН, Ф. И. О.).
    pub customer: Option<Customer>,

    /// Линейка товаров/услуг (минимум один элемент для чека).
    pub items: Vec<Items>,

    /// Телефон покупателя (если не заполнен в `customer`).
    pub phone: Option<String>,

    /// Email покупателя (если не заполнен в `customer`).
    pub email: Option<String>,

    /// Код системы налогообложения магазина (ОСН = 1, УСН = 2, …).
    pub tax_system_code: Option<i32>,

    /// Отраслевые реквизиты чека (алкоголь, маркировка, лекарства).
    pub receipt_industry_details: Option<Vec<Industry>>,

    /// Операционные реквизиты (применяются к всему чеку).
    pub receipt_operational_details: Option<Operation>,
}

/// Информация о покупателе.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Customer {
    /// Ф. И. О.
    pub full_name: Option<String>,
    /// ИНН физ. лица или юр. лица.
    pub inn: Option<String>,
    /// Email для отправки чека.
    pub email: Option<String>,
    /// Телефон покупателя (в любом РФ-формате).
    pub phone: Option<String>,
}

/// Позиция чека.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Items {
    /// Название товара/услуги.
    pub description: Option<String>,
    /// Цена за позицию × количество (`Amount`).
    pub amount: Amount,
    /// Код ставки НДС (0 – 6).
    pub vat_code: i32,
    /// Количество (может быть дробным, например 0.33 кг).
    pub quantity: Decimal,
    /// Единица измерения (`piece`, `kg`, `liter`, …).
    pub measure: Option<String>,
    /// Дробное количество при маркировке (ЕГАИС, Честный ЗНАК).
    pub mark_quantity: Option<MarkQuantity>,
    /// Признак предмета расчёта (`product`, `service`, …).
    pub payment_subject: Option<String>,
    /// Признак способа расчёта (`full_prepayment`, `credit`, …).
    pub payment_mode: Option<String>,
    /// Код страны происхождения (ISO 3166-1 alpha-2).
    pub country_of_origin_code: Option<String>,
    /// Номер ГТД/декларации.
    pub customs_declaration_number: Option<String>,
    /// Акциз (при алкогольной продукции, в рублях).
    pub excise: Option<String>,
    /// Код маркировки (base64 или UCN).
    pub product_code: Option<String>,
}

/// Расширенный блок количества маркированного товара.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct MarkQuantity {
    /// Числитель дроби.
    pub numerator: i32,
    /// Знаменатель дроби.
    pub denominator: i32,
    /// Детали кода маркировки.
    pub mark_code_info: Option<MarkCode>,
    /// Режим обработки кода (`strict`, `clarified`, …).
    pub mark_mode: Option<String>,
    /// Отраслевые реквизиты для предмета расчёта.
    pub payment_subject_industry_details: Option<Vec<Industry>>,
}

/// Представление разных форматов штрих-кода/UCN.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct MarkCode {
    pub mark_code_raw: Option<String>,
    pub unknown: Option<String>,
    pub ean_8: Option<String>,
    pub ean_13: Option<String>,
    pub itf_14: Option<String>,
    pub gs_10: Option<String>,
    pub gs_1m: Option<String>,
    pub short: Option<String>,
    pub fur: Option<String>,
    pub egais_20: Option<String>,
    pub egais_30: Option<String>,
}

/// Отраслевой признак/пара «ключ-значение».
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Industry {
    /// Код федерального закона/ведомства.
    pub federal_id: String,
    /// Дата документа (ISO 8601).
    pub document_date: String,
    /// Номер документа.
    pub document_number: String,
    /// Значение реквизита.
    pub value: String,
}

/// Операционные реквизиты чека.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Operation {
    pub operation_id: i32,
    pub value: String,
    pub created_at: String,
}

/// Данные авиаперелёта (Airline Addendum).
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Airline {
    /// Номер билета (талоны IATA 13 цифр).
    pub ticket_number: Option<String>,
    /// Код бронирования (PNR/Booking reference).
    pub booking_reference: Option<String>,
    /// Персональные данные пассажиров.
    pub passengers: Option<Vec<Passenger>>,
    /// Сегменты перелёта.
    pub legs: Option<Vec<Leg>>,
}

/// Пассажир.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Passenger {
    pub first_name: String,
    pub last_name: String,
}

/// Сегмент рейса.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Leg {
    pub departure_airport: String,
    pub destination_airport: String,
    /// Дата вылета (ISO 8601, локаль аэропорта).
    pub departure_date: String,
    /// IATA-код перевозчика (SU, DP, …).
    pub carrier_code: Option<String>,
}

/// Распределение суммы между суб-аккаунтами (сплит-платёж).
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Transfer {
    /// ID аккаунта-получателя.
    pub account_id: String,
    /// Сумма перевода этому получателю.
    pub amount: Amount,
    /// Комиссия платформы.
    pub platform_fee_amount: Option<Amount>,
    /// Текст комментария (виден получателю).
    pub description: Option<String>,
    /// Произвольные метаданные именно для этого трансфера.
    pub metadata: Option<HashMap<String, String>>,
}
