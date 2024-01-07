'use client'
import styles from './SendModal.module.scss'
import Image from 'next/image'
import CloseMnemonicIcon from '../../../public/close-mnemonic.png'
import TokenMoneySend from '../../../public/token-money-send.png'
import { useState } from 'react'

export interface Props {
  setHandlerSendModal: any
  sendTransfer: any
  disconnectWallet: any
}

const SendModal = ({ setHandlerSendModal, sendTransfer, disconnectWallet }: Props) => {
  const [sendAccountInput, setSendAccountInput] = useState<string>('')
  const [sendAmountInput, setSendAmountInput] = useState<number>(-10)

  const accountInput = (e: any) => {
    const lettersAndNumbers = /^[a-zA-Z0-9]+$/
    if (lettersAndNumbers.test(e.target.value)) {
      setSendAccountInput(e.target.value)
    } else if (Number(e.target.value.length) === 0) {
      setSendAccountInput('')
    }
  }



  const numberInput = (e: any) => {
    // if (Number(e.target.value) > Number(e.target.max)) {
    //   e.target.value = e.target.max
    // } else if (Number(e.target.value) < Number(e.target.min)) {
    //   e.target.value = e.target.min
    // }
    setSendAmountInput(e.target.value)
  }

  return (
    <>
      <div className={styles.backgroundModal}></div>
      <div className={styles.modalSelect}>
        <div
          className={styles.closeIcon}
          onClick={() => {
            setHandlerSendModal(false)
          }}
        >
          <Image src={CloseMnemonicIcon} alt={'close mnemonic image'}></Image>
        </div>
        <div className={styles.heroMoneySection}>
          <div className={styles.imageContainer}>
            <Image src={TokenMoneySend} alt={'Money Send Image'}></Image>
          </div>
          <div className={styles.heroMoneyText}>
                  <h3>Set up this transaction</h3>
                  <h4>Please write the account ID and the amount you want to send. </h4>
          </div>
        </div>
        <div className={styles.contentMoneyTransfer}>
          <div className={styles.inputsMoneyTransfer}>
            <form>
              <input type="text" placeholder="Account ID" onChange={accountInput} value={sendAccountInput} />
              <input type="number" onChange={numberInput} placeholder="Amount" />
            </form>
          </div>
          <div className={styles.buttonsMoneyTransfer}>
          <div className={styles.refuseButtonContainer}
            onClick={() => {
              setHandlerSendModal(false)
            } }
            >
              <p className={styles.closeButton}>Cancel</p>
          </div>
          <div
            className={styles.closeButtonContainer}
            onClick={() => { 
                sendTransfer(sendAccountInput, sendAmountInput) 
              }} 
            >
              <p className={`${styles.closeButton} ${sendAccountInput !== '' && sendAmountInput >= 0.00000001 ? '' : styles.closeDisabledButton}`}>Send tokens</p> 
          </div> 
          </div>
          {/* <input type="number" min="0.00000001" max="100" onChange={numberInput} placeholder="Amount" /> */}
          </div>
        </div>
        {/* <h3 className={styles.copyText}>Please write the account ID and the amount you want to send ! </h3> */}
    </>
  )
}

export default SendModal
