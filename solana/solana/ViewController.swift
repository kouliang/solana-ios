//
//  ViewController.swift
//  solana
//
//  Created by kou on 2024/12/28.
//

import UIKit

class ViewController: UIViewController {

    @IBOutlet var balanceLable: UILabel!
    @IBOutlet var balanceTextField: UITextField!
    
    @IBOutlet var transferLabel: UILabel!
    @IBOutlet var address: UITextField!
    @IBOutlet var amount: UITextField!
    
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
    }
    
    
    @IBAction func balanceRequest(_ sender: Any) {
        let addr = balanceTextField.text ?? ""
        
        let result = balance(addr)!
        let resultstr = String(cString: result)
        print(resultstr)
        
        balanceLable.text = resultstr
    }
    @IBAction func transferRequest(_ sender: Any) {
        let addr = address.text ?? ""
        let am = amount.text ?? ""
        
        let result = transfer_to(addr, am)!
        let resultstr = String(cString: result)
        print(resultstr)
        
        transferLabel.text = resultstr
    }
    
}

