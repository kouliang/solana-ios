//
//  SettingController.swift
//  solana
//
//  Created by kou on 2024/12/28.
//

import UIKit

class SettingController: UIViewController {
    
    @IBOutlet weak var url: UITextField!
    @IBOutlet weak var keypair: UITextField!
    
    @IBOutlet weak var urlLable: UILabel!
    @IBOutlet weak var keypairLable: UILabel!
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
    }
    
    
    @IBAction func url_verify(_ sender: Any) {
        let content = url.text ?? ""
        print(content)
        
        let result = test_rpc(content)!
        print(String(cString: result))

        urlLable.text = String(cString: result)
    }
    
    @IBAction func keypair_verify(_ sender: Any) {
        let content = keypair.text ?? ""
        print(content)
        
        let result = test_key_pair(content)!
        print(String(cString: result))
        
        keypairLable.text = String(cString: result)
    }
    
    @IBAction func saveconfig(_ sender: Any) {
        let url = url.text ?? ""
        let key = keypair.text ?? ""
        
        let result = save_config(url, key)!
        let resultstr = String(cString: result)
        print(resultstr)
        
        if resultstr == "OK" {
            self.dismiss(animated: true)
        }

    }
    
}
