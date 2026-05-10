const API_URL = "";

// ========================================
// ESTADO GLOBAL
// ========================================

let currentOrderId = null;

let pollingInterval = null;

// ========================================
// ASSINATURA MVP
// ========================================

function gerarSignature(payload) {

    // MVP TEMPORÁRIO
    // Depois será substituído
    // por HMAC SHA256 real

    return "teste";
}

// ========================================
// CRIAR PEDIDO
// ========================================

async function criarPedido() {

    const valorInput =
        document.getElementById("valor");

    const valor =
        parseFloat(valorInput.value);

    // ========================================
    // VALIDAÇÃO
    // ========================================

    if (!valor || valor <= 0) {

        alert("Informe um valor válido.");

        return;
    }

    // ========================================
    // PAYLOAD
    // ========================================

    const payload = {

        valor_brl: valor
    };

    const signature =
        gerarSignature(payload);

    try {

        // ========================================
        // REQUEST
        // ========================================

        const response = await fetch(
            `${API_URL}/orders`,
            {

                method: "POST",

                headers: {

                    "Content-Type":
                        "application/json",

                    "x-signature":
                        signature
                },

                body:
                    JSON.stringify(payload)
            }
        );

        // ========================================
        // ERROR HANDLER
        // ========================================

        if (!response.ok) {

            throw new Error(
                `Erro HTTP ${response.status}`
            );
        }

        // ========================================
        // JSON RESPONSE
        // ========================================

        const data =
            await response.json();

        console.log(
            "✅ Pedido criado:",
            data
        );

        currentOrderId = data.id;

        // ========================================
        // MOSTRAR RESULTADO
        // ========================================

        exibirResultado(data);

        // ========================================
        // INICIAR MONITORAMENTO
        // ========================================

        iniciarMonitoramento();

    } catch (error) {

        console.error(
            "❌ Erro ao criar pedido:",
            error
        );

        alert(
            "Erro ao criar pedido."
        );
    }
}

// ========================================
// EXIBIR RESULTADO
// ========================================

function exibirResultado(data) {

    const resultado =
        document.getElementById("resultado");

    resultado.classList.remove("hidden");

    // ========================================
    // VALOR XLM
    // ========================================

    document.getElementById(
        "valor_xlm"
    ).innerText = data.valor_xlm;

    // ========================================
    // MEMO
    // ========================================

    document.getElementById(
        "memo"
    ).value = data.memo;

    // ========================================
    // STATUS
    // ========================================

    const statusElement =
        document.getElementById("status");

    statusElement.innerText =
        "Aguardando pagamento...";

    statusElement.classList.remove(
        "confirmed"
    );

    statusElement.classList.add(
        "pending"
    );

    // ========================================
    // STELLAR LINK
    // ========================================

    const stellarLink =
        document.getElementById(
            "stellar_link"
        );

    stellarLink.href =
        data.payment_uri;

    stellarLink.innerText =
        "Abrir Wallet Stellar";

    // ========================================
    // QR CODE
    // ========================================

    const qrContainer =
        document.getElementById(
            "qrcode"
        );

    qrContainer.innerHTML = "";

    new QRCode(qrContainer, {

        text: data.payment_uri,

        width: 240,

        height: 240
    });
}

// ========================================
// MONITORAMENTO
// ========================================

function iniciarMonitoramento() {

    // ========================================
    // LIMPAR POLLING ANTERIOR
    // ========================================

    if (pollingInterval) {

        clearInterval(
            pollingInterval
        );
    }

    // ========================================
    // LOOP
    // ========================================

    pollingInterval = setInterval(
        async () => {

            try {

                const response =
                    await fetch(
                        `${API_URL}/orders`
                    );

                const orders =
                    await response.json();

                const order =
                    orders.find(
                        o =>
                            o.id ===
                            currentOrderId
                    );

                if (!order) {

                    console.log(
                        "Pedido não encontrado."
                    );

                    return;
                }

                atualizarStatus(order);

            } catch (error) {

                console.error(
                    "Erro monitoramento:",
                    error
                );
            }

        },

        3000
    );
}

// ========================================
// ATUALIZAR STATUS
// ========================================

function atualizarStatus(order) {

    const statusElement =
        document.getElementById("status");

    // ========================================
    // CONFIRMADO
    // ========================================

    if (
        order.status === "confirmed"
    ) {

        statusElement.innerText =
            "Pagamento Confirmado ✅";

        statusElement.classList.remove(
            "pending"
        );

        statusElement.classList.add(
            "confirmed"
        );

        statusElement.classList.add(
            "success-animation"
        );

        // ========================================
        // MOSTRAR HASH
        // ========================================

        mostrarTxHash(
            order.tx_hash
        );

        // ========================================
        // PARAR LOOP
        // ========================================

        clearInterval(
            pollingInterval
        );

        console.log(
            "✅ Pagamento confirmado!"
        );

    } else {

        statusElement.innerText =
            "Aguardando pagamento...";
    }
}

// ========================================
// MOSTRAR TX HASH
// ========================================

function mostrarTxHash(txHash) {

    let txElement =
        document.getElementById(
            "txhash"
        );

    // ========================================
    // CRIAR ELEMENTO
    // ========================================

    if (!txElement) {

        txElement =
            document.createElement(
                "div"
            );

        txElement.id = "txhash";

        txElement.className =
            "card";

        document
            .getElementById(
                "resultado"
            )
            .appendChild(txElement);
    }

    // ========================================
    // HTML
    // ========================================

    txElement.innerHTML = `

        <h3>
            🔗 Transação Confirmada
        </h3>

        <p style="
            word-break: break-all;
        ">
            ${txHash}
        </p>

        <a
            href="
https://stellar.expert/explorer/testnet/tx/${txHash}
"
            target="_blank"

            style="
                color:#38bdf8;
                text-decoration:none;
                font-weight:bold;
            "
        >
            Ver na Stellar Expert
        </a>
    `;
}

// ========================================
// DEBUG
// ========================================

console.log(
    "🚀 SlipPay Frontend iniciado"
);
