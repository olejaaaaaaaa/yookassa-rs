// Copyright (c) 2025 Oleg Pavlenko
//
// Модели объектов платежей YooKassa с подробной документацией по каждому полю.
// Описание составлено на основе официального справочника API YooKassa:
// https://yookassa.ru/developers/api#payment_object
//
// Все поля помечены `#[allow(dead_code)]`, чтобы компилятор не ругался,
// если некоторые из них не используются напрямую в коде.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::*;

/// Информация о платеже, возвращаемая YooKassa.
///
/// Представляет полную карточку платежа — от идентификатора до детальных сведений
/// о списаниях, распределениях и 3‑D Secure.
///
/// См. <https://yookassa.ru/developers/api#payment_object>.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ResponsePayment {
    /// Уникальный идентификатор платежа в YooKassa (пример: `29d9f131-000f-5000-9000-19420bb5e410`).
    pub id: String,

    /// Текущий статус платежа (`pending`, `waiting_for_capture`, `succeeded`, `canceled`).
    pub status: String,

    /// Запрошенная к списанию сумма и валюта.
    pub amount: Amount,

    /// Фактически зачисленная на расчётный счёт сумма (после вычета комиссии YooKassa).
    pub income_amount: Option<IncomeAmount>,

    /// Описание платежа, переданное магазином при создании.
    pub description: Option<String>,

    /// Реквизиты получателя платежа — идентификатор магазина и счёта.
    pub recipient: Recipient,

    /// Информация о способе оплаты, которым пользователь выполнил платёж.
    pub payment_method: Option<PaymentMethod>,

    /// Дата и время подтверждения платежа (формат RFC 3339), если платёж был захвачен.
    pub captured_at: Option<String>,

    /// Дата и время создания платежа (формат RFC 3339).
    pub created_at: String,

    /// Срок оплаты. После этой даты платёж автоматически перейдёт в статус `canceled`.
    pub expires_at: Option<String>,

    /// Данные, необходимые для подтверждения платежа (URL для перехода, тип confirmation и т.д.).
    pub confirmation: Option<Value>,

    /// Признак тестовой операции. `true`, если платёж создан в тестовом режиме.
    pub test: bool,

    /// Сумма уже возвращённых средств (при частичном/полном возврате).
    pub refunded_amount: Option<RefundedAmount>,

    /// Факт оплаты: `true`, если средства успешно списаны с плательщика.
    pub paid: bool,

    /// Возможность оформить возврат: `true`, если возврат доступен.
    pub refundable: bool,

    /// Статус фискализации чека (`pending`, `succeeded`, `canceled`), если подключена ККТ‑интеграция.
    pub receipt_registration: Option<String>,

    /// Пользовательские метаданные, переданные магазином (не более 16 KB).
    pub metadata: Option<HashMap<String, Value>>,

    /// Детали отмены платежа (кто отменил и почему), присутствуют при статусе `canceled`.
    pub cancellation_details: Option<CancellationDetails>,

    /// Детали авторизации при оплате картой (RRN, auth_code, 3‑D Secure).
    pub authorization_details: Option<AuthorizationDetails>,

    /// Распределение средств между суб‑аккаунтами при сплит‑платежах.
    pub transfers: Option<Transfers>,

    /// Информация о сделке (используется при связке «платёж + выплата»).
    pub deal: Option<Deal>,

    /// Идентификатор покупателя в системе магазина (merchant_customer_id).
    pub merchant_customer_id: Option<String>,

    /// Данные счёта, если платёж создан как выставление счёта (Invoice API).
    pub invoice_details: Option<InvoiceDetails>,
}

/// Список платежей с пагинацией.
///
/// Используется в ответах на запрос `GET /payments`.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ResponsePayments {
    /// Тип объекта ответа. Для списков всегда `list`.
    pub r#type: String,

    /// Курсор для получения следующей страницы результатов.
    pub next_cursor: String,

    /// Коллекция объектов [`ResponsePayment`].
    pub items: Vec<ResponsePayment>,
}

/// Параметры создания платежа.
///
/// Используется при вызове `POST /payments`.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RequestCreatePayments {
    /// Сумма к списанию. Обязательный параметр.
    pub amount: Option<Amount>,

    /// Комментарий или назначение платежа (до 128 символов).
    pub description: Option<String>,

    /// Параметры сценария подтверждения платежа (redirect, embedded, mobile_application и т.д.).
    pub confirmation: Option<Confirmation>,

    /// Данные о способе оплаты, если он ещё не сохранён в системе.
    pub payment_method_data: Option<PaymentMethodData>,
}

/// Детали сохранённого способа оплаты, использованного клиентом.
///
/// Присутствуют, если пользователь выбрал сохранённую карту, кошелёк и т.д.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentMethod {
    /// Тип способа оплаты (`bank_card`, `yoo_money`, `sbp` и т.д.).
    pub r#type: String,

    /// Идентификатор сохранённого способа оплаты.
    pub id: String,

    /// Признак сохранённого платёжного инструмента (`true` — сохранён).
    pub saved: bool,

    /// Человекочитаемое название, отображаемое пользователю (пример: «VISA‑4444»).
    pub title: String,

    /// Маскированный номер карты/кошелька/счёта.
    pub account_number: String,
}

/// Данные для создания нового способа оплаты.
///
/// Передаются вместо `payment_method_id`, если инструмент ещё не сохранён.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentMethodData {
    /// Тип создаваемого способа оплаты (`bank_card`, `sbp`, `mobile_balance` и т.д.).
    pub r#type: String,
}

/// Причина отмены платежа.
///
/// Заполняется YooKassa, когда платёж получает статус `canceled`.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct CancellationDetails {
    /// Сторона, инициировавшая отмену (`merchant`, `yookassa`, `payment_network`).
    pub party: String,

    /// Код или краткое описание причины отмены.
    pub reason: String,
}

/// Данные об авторизации платежа при оплате банковской картой.
/// Присутствуют только для этих способов оплаты: банковская карта, Mir Pay, SberPay, T-Pay.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorizationDetails {
    /// Retrieval Reference Number — уникальный идентификатор транзакции в платёжной системе.
    pub rrn: Option<String>,

    /// Код авторизации (auth_code), подтверждающий успешную авторизацию.
    pub auth_code: Option<String>,

    /// Данные о прохождении пользователем аутентификации по 3‑D Secure.
    pub three_d_secure: TreeDSecure,
}

/// Данные о прохождении пользователем аутентификации по 3‑D Secure для подтверждения платежа.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TreeDSecure {
    /// `true`, если YooKassa показала пользователю форму 3‑D Secure; `false`, если аутентификация не требовалась.
    pub applied: bool,
}

/// Данные о распределении денег — сколько и в какой магазин нужно перевести.
///
/// Присутствуют, если используется [сплитование платежей](https://yookassa.ru/developers/solutions-for-platforms/split-payments/basics).
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Transfers {
    /// Идентификатор счёта получателя (sub‑merchant).
    pub account_id: String,

    /// Сумма, перечисляемая получателю.
    pub amount: Amount,

    /// Статус перечисления (`pending`, `succeeded`, `canceled`).
    pub status: String,

    /// Комиссия платформы, удерживаемая из суммы перечисления.
    pub platform_fee_amount: Amount,

    /// Описание перечисления, переданное платформой.
    pub description: Option<String>,

    /// Дополнительные данные платформы, до 16 KB.
    pub metadata: Option<HashMap<String, Value>>,
}

/// Информация о сделке Safe Deal.
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Deal {
    /// Идентификатор сделки.
    pub id: String,
    /// Массив объектов с итоговыми расчётами.
    pub settlements: Vec<Value>,
}

/// Данные счёта (Invoice API).
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct InvoiceDetails {
    /// Идентификатор счёта.
    pub id: Option<String>,
}
